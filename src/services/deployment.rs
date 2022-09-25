use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Duration;

use anyhow::{bail, Context, Ok};
use anyhow::{Error, Result};
use aws_config::meta::region::RegionProviderChain;
use aws_iot_device_sdk::shadow;
use aws_sdk_greengrassv2::Client as Greengrassv2_Client;
use aws_sdk_greengrassv2::Region;
use aws_sdk_s3::Client as S3_Client;
use bytes::Bytes;
use once_cell::sync::Lazy;
use rumqttc::Publish;
use rumqttc::{AsyncClient, QoS};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use tokio::sync::mpsc::Sender;
use tokio::time;

use crate::services::{Service, SERVICES};
use crate::{config, ggcVersion};
const VERSION: &str = "0.0.0";

pub const CONFIGURATION_ARN_LOG_KEY_NAME: &str = "CONFIGURATION_ARN";
pub const DESIRED_STATUS_KEY: &str = "desiredStatus";
pub const FLEET_CONFIG_KEY: &str = "fleetConfig";
pub const GGC_VERSION_KEY: &str = "ggcVersion";
pub const DESIRED_STATUS_CANCELED: &str = "CANCELED";
pub const DEPLOYMENT_SHADOW_NAME: &str = "AWSManagedGreengrassV2Deployment";
pub const DEVICE_OFFLINE_MESSAGE: &str = "Device not configured to talk to AWS Iot cloud. ";
// + "Single device deployment is offline";
pub const SUBSCRIBING_TO_SHADOW_TOPICS_MESSAGE: &str = "Subscribing to Iot Shadow topics";

const NAME: &str = "DeploymentService";
pub struct Deployments {}

impl Service for Deployments {
    fn enable() {
        SERVICES.insert(NAME.into(), Self::new(NAME, VERSION));
    }
}

static DEPLOYSTATUS: DeployStates = DeployStates {
    mutex: Mutex::new(States::Deployment),
};

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

pub async fn connect_shadow(mqtt_client: &AsyncClient, thing_name: &str) -> Result<()> {
    let topic = format!("$aws/things/{thing_name}/shadow/name/{DEPLOYMENT_SHADOW_NAME}/#");
    mqtt_client.subscribe(&topic, QoS::AtMostOnce).await?;
    Ok(())
}

pub async fn disconnect_shadow(mqtt_client: AsyncClient, thing_name: &str) -> Result<()> {
    let topic = format!("$aws/things/{thing_name}/shadow/name/{DEPLOYMENT_SHADOW_NAME}/#");
    mqtt_client.unsubscribe(&topic).await?;
    Ok(())
}

fn assemble_payload(thing_name: &str, arn: &str, version: &str) -> Result<Value> {
    let version: u8 = version.parse()?;
    // for next status
    match DEPLOYSTATUS.get() {
        States::Deployment => Ok(json!({
          "shadowName": DEPLOYMENT_SHADOW_NAME,
          "thing_name": thing_name,
          "state": {
            "reported": {
              "ggcVersion": ggcVersion,
              "fleetConfigurationArnForStatus": arn,
              "status_details": {},
              "status": "IN_PROGRESS"
            }
          },
          "version": version
        })),
        States::Inprogress => Ok(json!({
          "shadowName": DEPLOYMENT_SHADOW_NAME,
          "thing_name": thing_name,
          "state": {
            "reported": {
              "ggcVersion": ggcVersion,
              "fleetConfigurationArnForStatus": arn,
              "status_details": {
                    "detailedStatus": "SUCCESSFUL"
              },
              "status": "SUCCEEDED"
            }
          },
          "version": version
        })),
        _ => Ok(json!("")),
    }
}

fn assemble_publish_content(v: Value) -> Result<Publish> {
    let shadow_version = v["version"].to_string();
    let v: Value = serde_json::from_str(v["state"]["fleetConfig"].as_str().unwrap())?;
    // "arn:aws:greengrass:<region>:<id>:configuration:thing/<name>:<version>"
    let configuration_arn = v["configurationArn"]
        .as_str()
        .context("Failed to get configuration arn.")?;
    let (other, version) = configuration_arn
        .rsplit_once(':')
        .context("Failed to get configuration version.")?;
    let (_, thing_name) = other
        .rsplit_once('/')
        .context("Failed to get thing name.")?;

    let topic = shadow::assemble_topic(
        shadow::Topic::Update,
        thing_name,
        Some(DEPLOYMENT_SHADOW_NAME),
    )
    .map_err(Error::msg)?;
    let payload = assemble_payload(thing_name, configuration_arn, &shadow_version)?;
    Ok(Publish {
        dup: false,
        qos: QoS::AtMostOnce,
        retain: false,
        pkid: 0,
        topic: topic.to_string(),
        payload: Bytes::from(payload.to_string()),
    })
}

pub async fn shadow_deployment(v: Publish, tx: Sender<Publish>) -> Result<()> {
    let v: Value = serde_json::from_slice(&v.payload)
        .context("Failed to deserialize deployment json file.")?;
    match DEPLOYSTATUS.get() {
        States::Deployment => {
            let value = assemble_publish_content(v)?;
            tx.send(value).await;
        }
        States::Inprogress => {
            let data: Value = serde_json::from_str(v["state"]["fleetConfig"].as_str().unwrap())?;

            // println!("[components] is {}", data["components"]);
            let mut map: HashMap<String, HashMap<String, serde_json::Value>> =
                serde_json::from_value(data["components"].to_owned())?;
            for (k, v) in map.drain().take(1) {
                component_deploy(
                    k,
                    v.get("version")
                        .context("Failed to find version feild.")?
                        .to_string()
                        .trim_matches('"')
                        .to_string(),
                )
                .await;
            }

            let value = assemble_publish_content(v)?;
            tx.send(value).await;
        }
        States::Succeed => {}
    }
    DEPLOYSTATUS.next();
    Ok(())
}

async fn component_deploy(name: String, version: String) -> Result<()> {
    let region = config::Config::global()
        .services
        .kernel
        .configuration
        .region
        .as_str();
    let region_provider = RegionProviderChain::first_try(Region::new(region)).or_default_provider();
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let ggv2_client = Greengrassv2_Client::new(&shared_config);
    let s3_client = S3_Client::new(&shared_config);

    // 1. list-components
    let arn = list_components(&ggv2_client, &name).await?;
    // 1.1. list-component-version
    let arn = list_component_version(&ggv2_client, &arn, &version).await?;
    // 2. get-component to get recipe.
    let recipe = get_component(&ggv2_client, &arn).await?;
    // 3. get-s3 for private component.
    println!("{}", recipe["Manifests"][0]["Artifacts"][0]["Uri"]);
    let uri = recipe["Manifests"][0]["Artifacts"][0]["Uri"]
        .as_str()
        .unwrap();
    get_s3_object(&s3_client, &uri).await;

    Ok(())
}
async fn list_components(client: &Greengrassv2_Client, name: &str) -> Result<String, Error> {
    let resp = client.list_components().send().await?;

    for component in resp.components().unwrap() {
        if name == component.component_name().unwrap() {
            return Ok(component.arn().unwrap().to_string());
        }
    }
    bail!("No such component.")
}
async fn list_component_version(
    client: &Greengrassv2_Client,
    arn: &str,
    version: &str,
) -> Result<String, Error> {
    let resp = client.list_component_versions().arn(arn).send().await?;

    for i in resp.component_versions().unwrap() {
        if version == i.component_version().unwrap() {
            return Ok(i.arn().unwrap().to_string());
        }
    }
    bail!("No such component version.")
}

async fn get_component(client: &Greengrassv2_Client, arn: &str) -> Result<Value, Error> {
    let resp = client.get_component().arn(arn).send().await?;

    println!("get_component:");

    println!(
        "   recipeOutputFormat:  {:?}",
        resp.recipe_output_format().unwrap()
    );
    let recipe = resp.recipe().unwrap();
    let recipe = recipe.to_owned().into_inner();
    let recipe = serde_json::from_slice(&recipe).unwrap();
    println!("   recipe:  {}", recipe);
    println!("   tags:  {:?}", resp.tags().unwrap());
    println!();

    println!();

    Ok(recipe)
}
async fn get_s3_object(client: &S3_Client, uri: &str) -> Result<(), Error> {
    let v: Vec<&str> = uri.splitn(4, '/').collect();

    // println!("{:?}", v);
    // println!("{:?}", v[0]);
    // println!("{:?}", v[3]);

    let resp = client.get_object().bucket(v[2]).key(v[3]).send().await?;
    let data = resp.body.collect().await;
    println!(
        "Data from downloaded object: {:?}",
        data // data.unwrap().into_bytes().slice(0..20)
    );

    Ok(())
}
