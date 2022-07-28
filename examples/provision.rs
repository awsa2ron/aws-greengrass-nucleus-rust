use anyhow::{Error, Result};
use aws_greengrass_nucleus as nucleus;
use rumqttc::{self, AsyncClient, Key, MqttOptions, QoS, Transport};
use serde_json::json;
use std::fs;
use tokio::{task, time};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let thing_name = "test";

    let mut mqtt_options = MqttOptions::new(thing_name, "endpoint", 8883);
    mqtt_options
        .set_transport(Transport::tls(
            fs::read("rootCA.pem")?,
            Some((
                fs::read("thingCert.crt")?,
                Key::RSA(fs::read("privKey.key")?),
            )),
            None,
        ));

    nucleus::easysetup::performSetup(thing_name, "region", true, "policy").await;

    let (client, mut eventloop) = AsyncClient::new(mqtt_options, 10);
    let topic = format!("$aws/things/{thing_name}/greengrassv2/health/json");
    let payload = nucleus::FleetStatus(thing_name);

    task::spawn(async move {
        client
            .publish(topic, QoS::AtLeastOnce, false, json!(payload).to_string())
            .await
            .unwrap();
    });

    while let Ok(notification) = eventloop.poll().await {
        println!("Received = {:?}", notification);
    }

    Ok(())
}
