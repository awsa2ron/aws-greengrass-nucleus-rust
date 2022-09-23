use anyhow::{Error, Result};
use aws_greengrass_nucleus::{
    config, mqtt,
    services::{self, deployment},
    Args, provisioning,
};
use aws_iot_device_sdk::{shadow, *};
use clap::Parser;
use rumqttc::{self, Event, Packet, Publish};
use tokio::sync::mpsc;
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();

    tracing_subscriber::fmt::init();
    if args.provision {
        provisioning::provision(&args).await?;
    }
    config::init(&args.init_config)?;
    let (mqtt_client, mut eventloop) = mqtt::init(&args.thing_name)?;
    let (tx, mut rx) = mpsc::channel(128);

    info!("Launching Nucleus...");
    services::start_services(tx.clone()).await?;
    info!("Launched Nucleus successfully.");
    deployment::connect_shadow(&mqtt_client, &args.thing_name).await?;
    while args.start {
        tokio::select! {
            Ok(event) = eventloop.poll() => { process(event, tx.clone()).await; }
            Some(msg) = rx.recv() => {
                let mqtt_client = mqtt_client.clone();
                tokio::spawn(async move {
                    println!("publish to {}",msg.topic);
                    mqtt_client.publish(msg.topic, msg.qos, false, msg.payload).await.unwrap();
                    // update effectiveConfig.yaml?
                });
            }
        }
    }
    Ok(())
}

async fn process(event: Event, tx: mpsc::Sender<Publish>) {
    println!("{:?}", event);
    if let Event::Incoming(Packet::Publish(v)) = event {
        match match_topic_type(&v.topic) {
            Ok(TopicType::NamedShadow) => {
                if shadow::match_topic(&v.topic).unwrap().shadow_op == shadow::Topic::UpdateDelta {
                    tokio::spawn(async move {
                        deployment::shadow_deployment(v, tx).await.unwrap();
                    });
                }
            }
            Ok(TopicType::Jobs) => {}
            _ => {}
        }
    }
    // Ok(())
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
