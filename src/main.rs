use anyhow::{Error, Result};
use aws_config::meta::region::RegionProviderChain;
use aws_greengrass_nucleus::{config, easysetup, mqtt, provisioning, util};
use aws_sdk_iot::{Client, PKG_VERSION};
use aws_types::region::Region;
use clap::Parser;
use rumqttc::{self, AsyncClient, Key, MqttOptions, QoS, Transport};
use serde::{Deserialize, Serialize};
use serde_json::json;
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
    #[clap(long)]
    aws_region: Option<String>,

    // (Optional) The path to the folder to use as the root for the AWS IoT Greengrass Core
    // software. Defaults to ~/.greengrass.
    #[clap(long, default_value = "/greengrass/v2")]
    root: String,

    // (Optional) The path to the configuration file that you use to run the AWS
    // IoT Greengrass Core software
    // software. Defaults to ~/.greengrass
    #[clap(long)]
    init_config: Option<String>,

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
    #[clap(long)]
    thing_policy_name: Option<String>,

    // (Optional) The name of the IAM role to use to acquire AWS credentials that let the device
    // interact with AWS services. If the role with this name doesn't exist in your AWS account, " the AWS
    // IoT Greengrass Core software creates it with the GreengrassV2TokenExchangeRoleAccess policy.
    // This role doesn't have access to your S3 buckets where you host component artifacts. So, you
    // must add permissions to your artifacts' S3 buckets and objects when you create a component.
    // Defaults to GreengrassV2TokenExchangeRole.
    #[clap(long)]
    tes_role_name: Option<String>,

    // (Optional) The name of the AWS IoT role alias that points to the IAM role that provides AWS
    // credentials for this device. If the role alias with this name doesn't exist in your AWS "
    // account, the
    // AWS IoT Greengrass Core software creates it and points it to the IAM role that you specify.
    // Defaults to GreengrassV2TokenExchangeRoleAlias.
    #[clap(long)]
    tes_role_alias_name: Option<String>,

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

    easysetup::performSetup(
        &thing_name,
        aws_region.unwrap_or("ap-southeast-1".into()),
        provision,
        thing_policy_name,
    )
    .await;

    let payload =
        json!(aws_greengrass_nucleus::services::status::uploadFleetStatusServiceData(&thing_name))
            .to_string();

    config::init();
    let endpoint = config::Config::global().endpoint.iot_ats.to_string();
    info!("Endpoint: {}", endpoint);

    // let payload = String::new();

    let rootDir = Path::new(".");
    let caFilePath = rootDir.join("rootCA.pem");
    let privKeyFilePath = rootDir.join("privKey.key");
    let certFilePath = rootDir.join("thingCert.crt");
    info!("{:?}", endpoint);

    let mut mqtt_options = MqttOptions::new(&thing_name, endpoint, 8883);
    mqtt_options
        .set_keep_alive(Duration::from_secs(30))
        .set_transport(Transport::tls(
            fs::read(caFilePath)?,
            Some((
                fs::read(certFilePath)?,
                Key::RSA(fs::read(privKeyFilePath)?),
            )),
            None,
        ));

    let (client, mut eventloop) = AsyncClient::new(mqtt_options, 10);
    // let topic = format!("greengrassv2/health/json");
    let topic = format!("$aws/things/{thing_name}/greengrassv2/health/json");
    info!("Send {payload} to {topic}");
    tokio::join!(
        // util::publish(client, "hello/world"), // easysetup::createThing(client, &name, &name),
        mqtt::publish(client, payload.into(), topic, QoS::AtLeastOnce, true)
    );

    while let Ok(notification) = eventloop.poll().await {
        println!("Received = {:?}", notification);
    }

    Ok(())
}
