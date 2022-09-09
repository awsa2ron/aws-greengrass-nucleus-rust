#![allow(non_snake_case)]
#![allow(unused)]

use anyhow::{Error, Result};
use aws_greengrass_nucleus::{config, easysetup, mqtt, services::deployment, Args};
use aws_iot_device_sdk::{shadow, *};
use clap::Parser;
use rumqttc::{self, Event, Packet, Publish};
use tokio::sync::mpsc;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();

    tracing_subscriber::fmt::init();
    config::init();
    let (mqtt_client, mut eventloop) = mqtt::init(&args.thing_name)?;
    easysetup::perform_setup(&mqtt_client, &args).await;
    deployment::connect_shadow(&mqtt_client, &args.thing_name).await;

    let (tx, mut rx) = mpsc::channel(128);
    loop {
        tokio::select! {
            Ok(event) = eventloop.poll() => { process(event, tx.clone()).await; }
            Some(msg) = rx.recv() => {
                let mqtt_client = mqtt_client.clone();
                tokio::spawn(async move {
                    mqtt_client.publish(msg.topic, msg.qos, false, msg.payload).await.unwrap();
                });
            }
        }
    }
}

async fn process(event: Event, tx: mpsc::Sender<Publish>) {
    println!("{:?}", event);
    if let Event::Incoming(Packet::Publish(v)) = event {
        match match_topic_type(&v.topic) {
            Ok(TopicType::NamedShadow) => {
                if shadow::match_topic(&v.topic).unwrap().shadow_op == shadow::Topic::UpdateDelta {
                    tokio::spawn(async move {
                        deployment::resp_shadow_delta(v, tx).await;
                    });
                }
            }
            Ok(TopicType::Jobs) => {}
            _ => {}
        }
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
