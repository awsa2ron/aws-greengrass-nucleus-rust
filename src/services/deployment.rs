use std::sync::Mutex;
use std::time::Duration;

use anyhow::{Error, Result};
use aws_iot_device_sdk::shadow;
use aws_sdk_greengrassv2::{Client, Region};
use bytes::Bytes;
use once_cell::sync::Lazy;
use rumqttc::Publish;
use rumqttc::{AsyncClient, QoS};
use serde_json::json;
use serde_json::Value;
use tokio::sync::mpsc::Sender;
use tokio::time;

use crate::services::{Service, SERVICES};

// const long TIMEOUT_FOR_SUBSCRIBING_TO_TOPICS_SECONDS = Duration.ofMinutes(1).getSeconds();
// const long TIMEOUT_FOR_PUBLISHING_TO_TOPICS_SECONDS = Duration.ofMinutes(1).getSeconds();
// const long WAIT_TIME_TO_SUBSCRIBE_AGAIN_IN_MS = Duration.ofMinutes(2).toMillis();
// const Logger logger = LogManager.getLogger(ShadowDeploymentListener.class);
pub const CONFIGURATION_ARN_LOG_KEY_NAME: &str = "CONFIGURATION_ARN";
pub const DESIRED_STATUS_KEY: &str = "desiredStatus";
pub const FLEET_CONFIG_KEY: &str = "fleetConfig";
pub const GGC_VERSION_KEY: &str = "ggcVersion";
pub const DESIRED_STATUS_CANCELED: &str = "CANCELED";
pub const DEPLOYMENT_SHADOW_NAME: &str = "AWSManagedGreengrassV2Deployment";
pub const DEVICE_OFFLINE_MESSAGE: &str = "Device not configured to talk to AWS Iot cloud. ";
// + "Single device deployment is offline";
pub const SUBSCRIBING_TO_SHADOW_TOPICS_MESSAGE: &str = "Subscribing to Iot Shadow topics";

const VERSION: &str = "0.0.0";
const NAME: &str = "DeploymentService";
pub struct Deployments {}

impl Service for Deployments {
    fn enable() {
        SERVICES.insert(NAME.into(), Self::new(NAME, VERSION));
    }
}

#[derive(Debug, Copy, Clone)]
enum States {
    Deployment = 0,
    Inprogress,
    Succeed,
}
struct DeployStates {
    mutex: Mutex<States>,
}

impl DeployStates {
    fn get(&self) -> States {
        let lock = self.mutex.lock().unwrap();
        *lock
    }
    fn reset(&self) {
        let mut lock = self.mutex.lock().unwrap();
        *lock = States::Deployment;
    }
    fn next(&self) {
        let mut lock = self.mutex.lock().unwrap();
        match *lock {
            States::Deployment => *lock = States::Inprogress,
            States::Inprogress => *lock = States::Succeed,
            States::Succeed => self.reset(),
        }
    }
}
static DEPLOYSTATUS: DeployStates = DeployStates {
    mutex: Mutex::new(States::Deployment),
};

pub struct Deployment {
    content: String,
}
impl Deployment {
    pub fn new() -> InProgress {
        InProgress {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

pub struct InProgress {
    content: String,
}
impl InProgress {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn request_review(self) -> Succeed {
        Succeed {
            content: self.content,
        }
    }
}

pub struct Succeed {
    content: String,
}
impl Succeed {
    pub fn approve(self) -> Deployment {
        Deployment {
            content: self.content,
            // mutex: Mutex::new(0),
        }
    }
}

/// Todo: fix bug in aws-iot-device-sdk-rust
/// Match shadow topic without suffix.
pub async fn connect_shadow(mqtt_client: AsyncClient, thing_name: &str) {
    let topic = format!("$aws/things/{thing_name}/shadow/name/AWSManagedGreengrassV2Deployment/update/#");
    mqtt_client
        .subscribe(&topic, QoS::AtMostOnce)
        .await
        .unwrap();
}

pub async fn disconnect_shadow(mqtt_client: AsyncClient, thing_name: &str) {
    let topic = format!("$aws/things/{thing_name}/shadow/name/AWSManagedGreengrassV2Deployment/update/#");
    mqtt_client.unsubscribe(&topic).await.unwrap();
}

fn assemble_payload(thing_name: &str, arn: &str, version: &str, next: bool) -> Value {
    let version: u8 = version.parse().unwrap();
    if next {
        json!({
          "shadowName": "AWSManagedGreengrassV2Deployment",
          "thingName": thing_name,
          "state": {
            "reported": {
              "ggcVersion": "2.5.6",
              "fleetConfigurationArnForStatus": arn,
              "statusDetails": {},
              "status": "IN_PROGRESS"
            }
          },
          "version": version
        })
    } else {
        json!({
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
          "version": version
        })
    }
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
async fn get_component_recipe(client: &Client, bucket: &str, region: &str) { //} -> Result<(), Error> {
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

    // Ok(())
}

pub async fn resp_shadow_delta(v: Publish, tx: Sender<Publish>) {
    println!("const status is: {:#?}", DEPLOYSTATUS.get());

    let v: Value = serde_json::from_slice(&v.payload).unwrap();
    // version
    let shadow_version = v["version"].clone().to_string();
    // ["state"]["fleetConfig"]
    let v = v["state"]["fleetConfig"]
        .to_string()
        .replace("\\", "")
        .trim_matches('"')
        .to_string();
    // ["state"]["fleetConfig"]["configurationArn"]
    let v: Value = serde_json::from_str(&v).unwrap();
    // "arn:aws:greengrass:region:id:configuration:thing/name:deployment_version"
    let configuration_arn = v["configurationArn"]
        .to_string()
        .trim_matches('"')
        .to_string();
    let (arn, version) = configuration_arn.rsplit_once(':').unwrap();
    let (_, thing_name) = arn.rsplit_once('/').unwrap();
    let topic = shadow::assemble_topic(
        shadow::Topic::Update,
        thing_name,
        Some("AWSManagedGreengrassV2Deployment"),
    )
    .unwrap();

    match DEPLOYSTATUS.get() {
        States::Deployment => {
            let payload = assemble_payload(thing_name, &configuration_arn, &shadow_version, true).to_string();
            let value = Publish {
                dup: false,
                qos: QoS::AtMostOnce,
                retain: false,
                pkid: 0,
                topic: topic.to_string(),
                payload: Bytes::from(payload),
            };
            tx.send(value).await;
        }
        States::Inprogress => {
            let payload = assemble_payload(thing_name, &configuration_arn, &shadow_version, false).to_string();
            let value = Publish {
                dup: false,
                qos: QoS::AtMostOnce,
                retain: false,
                pkid: 0,
                topic: topic.to_string(),
                payload: Bytes::from(payload),
            };
            time::sleep(Duration::from_secs(3)).await;
            tx.send(value).await;
        }
        States::Succeed => {}
    }
    DEPLOYSTATUS.next();
    // Ok(())
}
