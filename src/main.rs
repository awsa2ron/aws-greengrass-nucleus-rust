use anyhow::{Error, Result};
use aws_greengrass_nucleus::services::deployment;
use aws_greengrass_nucleus::{config, easysetup, http, mqtt, Args};
use aws_iot_device_sdk::shadow;
use clap::Parser;
use rumqttc::{self, Event, Packet};
use tokio::sync::mpsc::Sender;
use tokio::sync::mpsc;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();

    tracing_subscriber::fmt::init();
    config::init();
    let (mqtt_client, mut eventloop) = mqtt::init(&args.thing_name)?;
    let http_client = http::init(&args.aws_region).await.unwrap();
    easysetup::perform_setup(http_client, mqtt_client.clone(), &args).await;
    deployment::connect_shadow(mqtt_client.clone(), &args.thing_name).await;

    let (tx, mut rx) = mpsc::channel::<String>(128);
    loop {
        tokio::select! {
            Ok(event) = eventloop.poll() => { process(event, tx.clone()).await; }
            Some(msg) = rx.recv() => { println!("{msg}"); }
        }
    }
}

async fn process(e: Event, tx: Sender<String>) {
    println!("{:?}", e);
    match e {
        Event::Incoming(Packet::Publish(v)) => {
            if let Ok(shadow) = shadow::match_topic(&v.topic) {
                match shadow.shadow_op {
                    shadow::Topic::UpdateDelta => {}
                    _ => {}
                }
            }
        }
        rumqttc::Event::Incoming(_) => {}
        rumqttc::Event::Outgoing(_) => {} // if let Some(v) = rx.recv().await {
    }
}

#[allow(unused)]
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tdd() {
        let data = r#"
        "#;
    }
}
