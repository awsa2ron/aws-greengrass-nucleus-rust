#![allow(unused)]
use anyhow::{Error, Result};
use aws_greengrass_nucleus::{config, easysetup};
use aws_sdk_iot::{Client, PKG_VERSION};
use clap::Parser;
use rumqttc::{self, AsyncClient, Key, MqttOptions, QoS, Transport};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use std::fs;
use std::path::Path;
use std::time::Duration;
use tokio::{task, time};
use tracing::{debug, event, info, span, Level};
use tracing_subscriber;

// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // The AWS Region to use. The AWS IoT Greengrass Core software uses this Region
    // to retrieve or create the AWS resources that it requires
    #[clap(long, default_value = "ap-southeast-1")]
    aws_region: String,

    // (Optional) The path to the folder to use as the root for the AWS IoT Greengrass Core
    // software. Defaults to ~/.greengrass.
    #[clap(long, default_value = "/greengrass/v2")]
    root: String,

    // (Optional) The path to the configuration file that you use to run the AWS
    // IoT Greengrass Core software
    // software. Defaults to ~/.greengrass
    #[clap(long, default_value = "~/.greengrass")]
    init_config: String,

    // (Optional) Specify true or false. If true, the AWS IoT Greengrass Core software registers this
    // device as an AWS IoT thing, and provisions the AWS resources that the software requires. The
    // software provisions an AWS IoT thing, (optional) an AWS IoT thing group, a Thing Policy, an
    // IAM role, and an AWS IoT role alias. Defaults to false.
    #[clap(long)]
    provision: bool,

    // (Optional) The name of the AWS IoT thing that you register as this core device.
    // If the thing with
    // this name doesn't exist in your AWS account, the AWS IoT Greengrass Core software creates it.
    //Defaults to GreengrassV2IotThing_ plus a random UUID.
    #[clap(short, long)]
    thing_name: String,

    // (Optional) The name of the AWS IoT thing group where you add this core
    // device's AWS IoT thing.
    // If a deployment targets this thing group, this core device receives that deployment when it
    // connects to AWS IoT Greengrass. If the thing group with this name doesn't exist in your AWS
    // account, the AWS IoT Greengrass Core software creates it. Defaults to no thing group.
    #[clap(long)]
    thing_group_name: Option<String>,

    // (Optional) The name of the AWS IoT Policy to attach to the core device's
    // AWS IoT thing.
    // If specified, then the supplied thing_policy_name is attached to the provisioned IoT Thing.
    // Otherwise a policy called GreengrassV2IoTThingPolicy is used instead. If the policy with
    // this name doesn't exist in your AWS account, the AWS IoT Greengrass Core software creates it
    // with a default policy document.
    #[clap(long, default_value = "GreengrassV2IoTThingPolicy")]
    thing_policy_name: String,

    // (Optional) The name of the IAM role to use to acquire AWS credentials that let the device
    // interact with AWS services. If the role with this name doesn't exist in your AWS account, " the AWS
    // IoT Greengrass Core software creates it with the GreengrassV2TokenExchangeRoleAccess policy.
    // This role doesn't have access to your S3 buckets where you host component artifacts. So, you
    // must add permissions to your artifacts' S3 buckets and objects when you create a component.
    // Defaults to GreengrassV2TokenExchangeRole.
    #[clap(long, default_value = "GreengrassV2TokenExchangeRole")]
    tes_role_name: String,

    // (Optional) The name of the AWS IoT role alias that points to the IAM role that provides AWS
    // credentials for this device. If the role alias with this name doesn't exist in your AWS "
    // account, the
    // AWS IoT Greengrass Core software creates it and points it to the IAM role that you specify.
    // Defaults to GreengrassV2TokenExchangeRoleAlias.
    #[clap(long, default_value = "GreengrassV2TokenExchangeRoleAlias")]
    tes_role_alias_name: String,

    // (Optional) Specify true or false. If true, then the AWS IoT Greengrass Core software sets
    // itself up as a system service that runs when this device boots. The system service name is "
    // greengrass.
    // Defaults to false.
    #[clap(long)]
    setup_system_service: bool,

    // (Optional) The name of ID of the system user and group that the AWS IoT Greengrass Core
    // software uses to run components. This argument accepts the user and group separated by a
    // colon, where the group is optional. For example, you can specify ggc_user:ggc_group or
    // ggc_user.
    // * If you run as root, this defaults to the user and group that the config file defines. If the config
    // file doesn't define a user and group, this defaults to ggc_user:ggc_group. If ggc_user or
    // ggc_group don't exist, the software creates them.
    // * If you run as a non_root user, the AWS IoT Greengrass Core software uses that user to run "
    // components.
    // * If you don't specify a group, the AWS IoT Greengrass Core software uses the primary group
    // of the system user
    #[clap(long)]
    component_default_user: Option<String>,

    // (Optional) Specify true or false. If true, the AWS IoT Greengrass Core software retrieves and
    // deploys the Greengrass CLI component. Specify true to set up this core
    // device for local development. Specify false to set up this core device in a production
    // environment. Defaults to false.
    #[clap(long)]
    deploy_dev_tools: bool,

    // (Optional) Specify true or false. If true, the AWS IoT Greengrass Core software runs setup steps,
    // (optional) provisions resources, and starts the software. If false, the software runs only setup
    // steps and (optional) provisions resources. Defaults to true.
    #[clap(long)]
    start: bool,

    // (Optional) Path of a plugin jar file. The plugin will be included as "
    // trusted plugin in nucleus. Specify multiple times for including multiple plugins.;
    #[clap(long)]
    trusted_plugin: Option<String>,
}

// #[tokio::main]
// #[cfg(feature = "use-rustls")]
#[tokio::main]
async fn main() -> Result<(), Error> {
    let Args {
        aws_region,
        root,
        init_config,
        provision,
        thing_name,
        thing_group_name,
        thing_policy_name,
        tes_role_name,
        tes_role_alias_name,
        setup_system_service,
        component_default_user,
        deploy_dev_tools,
        start,
        trusted_plugin,
    } = Args::parse();

    tracing_subscriber::fmt::init();

    easysetup::performSetup(&thing_name, &aws_region, provision, &thing_policy_name).await;

    config::init();
    let endpoint = config::Config::global().endpoint.iot_ats.to_string();
    info!("Endpoint: {}", endpoint);

    let root_dir = Path::new(".");
    let ca_file_path = root_dir.join("rootCA.pem");
    let priv_key_file_path = root_dir.join("privKey.key");
    let cert_file_path = root_dir.join("thingCert.crt");
    info!("{:?}", endpoint);

    let mut mqtt_options = MqttOptions::new(&thing_name, endpoint, 8883);
    mqtt_options
        .set_keep_alive(Duration::from_secs(30))
        .set_transport(Transport::tls(
            fs::read(ca_file_path)?,
            Some((
                fs::read(cert_file_path)?,
                Key::RSA(fs::read(priv_key_file_path)?),
            )),
            None,
        ));

    let (client, mut eventloop) = AsyncClient::new(mqtt_options, 10);

    if provision {
        let topic = format!("$aws/things/{thing_name}/greengrassv2/health/json");
        let payload = json!(aws_greengrass_nucleus::services::status::upload_fss_data(
            &thing_name
        ))
        .to_string();
        info!("Send {payload} to {topic}");
        client
            .publish(topic, QoS::AtLeastOnce, false, payload)
            .await
            .unwrap();
    }

    let topic = format!("$aws/things/{thing_name}/shadow/name/AWSManagedGreengrassV2Deployment/#");
    client.subscribe(&topic, QoS::AtMostOnce).await.unwrap();

    while let Ok(notification) = eventloop.poll().await {
        println!("Received = {:?}", notification);
        match notification {
            rumqttc::Event::Incoming(rumqttc::Packet::Publish(v)) => {
                // println!("{:?}", v.dup);
                println!("QoS is {:?}", v.qos);
                println!("Retain is {:?}", v.retain);
                // println!("ID is {:?}", v.pkid);
                println!("Topic is {:?}", v.topic);
                // println!("{:#?}", v.payload);
                if v.topic.rfind("delta") != None {
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
                        client.unsubscribe(&topic).await.unwrap();
                        time::sleep(Duration::from_secs(3)).await;
                        println!("{arn}|{version}");
                        let c = client.clone();
                        let n = thing_name.clone();
                        let a = configuration_arn.clone();
                        let v = shadow_version.to_string();
                        let s = "IN_PROGRESS".to_string();
                        task::spawn(async move {
                            update(c, n, a, v, s).await;
                            // time::sleep(Duration::from_secs(3)).await;
                        });
                    }
                }
            }
            rumqttc::Event::Incoming(_) => {}
            rumqttc::Event::Outgoing(_) => {}
        }
    }

    Ok(())
}

async fn update(
    client: AsyncClient,
    thing_name: String,
    arn: String,
    version: String,
    status: String,
) {
    let topic =
        format!("$aws/things/{thing_name}/shadow/name/AWSManagedGreengrassV2Deployment/update");
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

    time::sleep(Duration::from_secs(3)).await;

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tdd() {
        let data = r#"
        "#;
    }
}
