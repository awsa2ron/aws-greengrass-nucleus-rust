#![allow(unused)]
use anyhow::{Error, Result};
use aws_config::meta::region::RegionProviderChain;
use aws_greengrass_nucleus::services::deployment;
use aws_greengrass_nucleus::{config, easysetup, http, mqtt, Args};
use aws_iot_device_sdk::shadow;
use aws_iot_device_sdk::shadow::ThingShadow;
use aws_sdk_greengrassv2::{Client, Region};
use clap::Parser;
use rumqttc::Publish;
use rumqttc::{self, AsyncClient, Event, Key, MqttOptions, Packet, QoS, Transport};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use std::fs;
use std::path::Path;
use std::time::Duration;
use tokio::sync::mpsc::Sender;
use tokio::sync::{mpsc, oneshot};
use tokio::{task, time};
use tracing::{debug, event, info, span, Level};
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
    Ok(())
}

async fn process(e: Event, tx: Sender<String>) {
    println!("{:?}", e);
    match e {
        rumqttc::Event::Incoming(rumqttc::Packet::Publish(v)) => {
            let shadow = shadow::match_topic(&v.topic).unwrap();
            if shadow.shadow_op == shadow::Topic::UpdateDelta {
                let v: Value = serde_json::from_slice(&v.payload).unwrap();
                let shadow_version = v["version"].clone();
                let v = v["state"]["fleetConfig"]
                    .to_string()
                    .replace("\\", "")
                    .trim_matches('"')
                    .to_string();
                let v: Value = serde_json::from_str(&v).unwrap();
                println!("{}", v["configurationArn"]);
                let configuration_arn = v["configurationArn"]
                    .to_string()
                    .trim_matches('"')
                    .to_string();
                if let Some((arn, version)) = configuration_arn.rsplit_once(':') {
                    tx.send("How are you?".to_string()).await;
                    // time::sleep(Duration::from_secs(3)).await;
                    println!("{arn}|{version}");
                    // let c = mqtt_client.clone();
                    // let n = args.thing_name.clone();
                    // let a = configuration_arn.clone();
                    // let v = shadow_version.to_string();
                    // let s = "IN_PROGRESS".to_string();
                    // task::spawn(async move {
                    //     update(c, n, a, v, s).await;
                    //     // time::sleep(Duration::from_secs(3)).await;
                    // });
                }
            }
        }
        rumqttc::Event::Incoming(_) => {}
        rumqttc::Event::Outgoing(_) => {} // if let Some(v) = rx.recv().await {
    }

    // }
}

async fn update(
    client: AsyncClient,
    thing_name: String,
    arn: String,
    version: String,
    status: String,
) {
    // let topic = format!("$aws/things/{thing_name}/shadow/name/AWSManagedGreengrassV2Deployment/update");
    let topic = shadow::get_topic(
        shadow::Topic::Update,
        &thing_name,
        Some("AWSManagedGreengrassV2Deployment"),
    )
    .unwrap()
    .to_string();
    println!("{topic}");

    let version: u8 = version.parse().unwrap();

    let payload = json!({
      "shadowName": "AWSManagedGreengrassV2Deployment",
      "thingName": thing_name,
      "state": {
        "reported": {
          "ggcVersion": "2.5.6",
          "fleetConfigurationArnForStatus": arn,
          "statusDetails": {
            // if status {
            //     "detailedStatus": "SUCCESSFUL"
            // }
          },
          "status": status
        }
      },
      "version": version
    });
    println!("{payload}");

    client
        .publish(&topic, QoS::AtLeastOnce, false, payload.to_string())
        .await
        .unwrap();

    seeking().await;

    let payload = json!({
      "shadowName": "AWSManagedGreengrassV2Deployment",
      "thingName": thing_name,
      "state": {
        "reported": {
          "ggcVersion": "2.5.6",
          "fleetConfigurationArnForStatus": arn,
          "statusDetails": {
                "detailedStatus": "SUCCESSFUL"
          },
          "status": "SUCCEEDED"
        }
      },
      "version": version + 1
    });

    client
        .publish(&topic, QoS::AtLeastOnce, false, payload.to_string())
        .await
        .unwrap();
}

async fn seeking() {
    // let region_provider = RegionProviderChain::first_try(Region::new("ap-southeast-1"))
    //     .or_default_provider()
    //     .or_else(Region::new("ap-southeast-1"));

    // let shared_config = aws_config::from_env().region(region_provider).load().await;
    // let client = Client::new(&shared_config);

    // get recipe
    // get artifacts
    // let resp = client
    //     .get_component_version_artifact()
    //     .send()
    //     .await
    //     .unwrap();
}

// Get a component.
async fn get_component_recipe(client: &Client, bucket: &str, region: &str) -> Result<(), Error> {
    // let constraint = BucketLocationConstraint::from(region);
    // let cfg = CreateBucketConfiguration::builder()
    //     .location_constraint(constraint)
    //     .build();

    // client
    //     .create_bucket()
    //     .create_bucket_configuration(cfg)
    //     .bucket(bucket)
    //     .send()
    //     .await?;
    // println!("Created bucket.");

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tdd() {
        let data = r#"
        "#;
    }
}
