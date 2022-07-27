use anyhow::{Error, Result};
use aws_greengrass_nucleus as nucleus;
use rumqttc::{self, AsyncClient, Key, MqttOptions, QoS, Transport};
use serde_json::json;
use std::fs;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let thing_name = "test";
    nucleus::easysetup::performSetup(
        thing_name,
        "ap-southeast-1",
        true,
        "policy",
    )
    .await;

    let payload = json!(nucleus::uploadFleetStatusServiceData(&thing_name));

    let mut mqtt_options = MqttOptions::new(thing_name, "endpoint", 8883);
    mqtt_options
        .set_keep_alive(Duration::from_secs(30))
        .set_transport(Transport::tls(
            fs::read("rootCA.pem")?,
            Some((
                fs::read("thingCert.crt")?,
                Key::RSA(fs::read("privKey.key")?),
            )),
            None,
        ));

    let (client, mut eventloop) = AsyncClient::new(mqtt_options, 10);
    let topic = format!("$aws/things/{thing_name}/greengrassv2/health/json");
    tokio::join!(nucleus::publish(
        client,
        payload.to_string().into(),
        topic,
        QoS::AtLeastOnce,
        true
    ));

    while let Ok(notification) = eventloop.poll().await {
        println!("Received = {:?}", notification);
    }

    Ok(())
}
