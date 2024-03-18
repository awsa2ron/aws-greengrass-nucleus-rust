#![allow(non_snake_case)]
use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Ok, Result};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_greengrassv2::Region;
use aws_sdk_iot::Client;
// use greengrassv2 as ggv2;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use tracing::{debug, event, info, span, Level};

use crate::Args;

// #[derive(Serialize, Deserialize, Debug)]
// pub struct SystemConfiguration {
//     pub certificateFilePath: PathBuf,
//     pub privateKeyPath: PathBuf,
//     pub rootCaPath: PathBuf,
//     pub rootpath: PathBuf,
//     pub thingName: String,
// }

// pub static SYSCONFIG: OnceCell<SystemConfiguration> = OnceCell::new();

// impl SystemConfiguration {
//     pub fn global() -> &'static SystemConfiguration {
//         SYSCONFIG
//             .get()
//             .expect("System configuration is not initialized")
//     }
//     fn update(
//         thingName: String,
//         certificateFilePath: PathBuf,
//         privateKeyPath: PathBuf,
//         rootCaPath: PathBuf,
//         rootpath: PathBuf,
//     ) -> Result<SystemConfiguration, anyhow::Error> {
//         let ret = SystemConfiguration {
//             certificateFilePath,
//             privateKeyPath,
//             rootCaPath,
//             rootpath,
//             thingName,
//         };
//         Ok(ret)
//     }
// }

// /**
//  * Updates the system configuration values in kernel config as per the given {@link SystemConfiguration}.
//  * @param systemConfiguration {@link SystemConfiguration}
//  * @param updateBehavior Update behavior indicating either merge or replace
//  */
// pub fn updateSystemConfiguration(
//     thingName: &str,
//     caFilePath: PathBuf,
//     privKeyFilePath: PathBuf,
//     certFilePath: PathBuf,
//     rootpath: PathBuf,
// ) {
//     let sysConfig = SystemConfiguration::update(
//         thingName.to_string(),
//         certFilePath,
//         privKeyFilePath,
//         caFilePath,
//         rootpath,
//     )
//     .unwrap();
//     SYSCONFIG.set(sysConfig).unwrap();
// }

// #[derive(Debug)]
// pub struct NucleusConfiguration {
//     awsRegion: String,
//     iotCredentialsEndpoint: String,
//     iotDataEndpoint: String,
//     iotRoleAlias: String,
// }
// pub static NUCLEUSCONFIG: OnceCell<NucleusConfiguration> = OnceCell::new();

// impl NucleusConfiguration {
//     pub fn global() -> &'static NucleusConfiguration {
//         NUCLEUSCONFIG
//             .get()
//             .expect("System configuration is not initialized")
//     }

//     fn update(
//         awsRegion: String,
//         iotCredentialsEndpoint: String,
//         iotDataEndpoint: String,
//         iotRoleAlias: String,
//     ) -> Result<NucleusConfiguration, anyhow::Error> {
//         let ret = NucleusConfiguration {
//             awsRegion,
//             iotCredentialsEndpoint,
//             iotDataEndpoint,
//             iotRoleAlias,
//         };
//         Ok(ret)
//     }
// }
// /**
//  * Updates the nucleus configuration value in kernel config as per the given {@link NucleusConfiguration}.
//  * @param nucleusConfiguration {@link NucleusConfiguration}
//  * @param updateBehavior Update behavior indicating either merge or replace
//  */
// pub fn updateNucleusConfiguration(
//     awsRegion: String,
//     iotCredentialsEndpoint: String,
//     iotDataEndpoint: String,
//     iotRoleAlias: String,
// ) {
//     let nucleusConfig = NucleusConfiguration::update(
//         awsRegion.to_string(),
//         iotCredentialsEndpoint,
//         iotDataEndpoint,
//         iotRoleAlias,
//     )
//     .unwrap();
//     NUCLEUSCONFIG.set(nucleusConfig).unwrap();
// }
// pub struct ProvisionConfiguration {
//     systemConfiguration: OnceCell<SystemConfiguration>,
//     nucleusConfiguration: OnceCell<NucleusConfiguration>,
// }

// impl ProvisionConfiguration {}

// const GG_TOKEN_EXCHANGE_ROLE_ACCESS_POLICY_SUFFIX: &str = "Access";
const GG_TOKEN_EXCHANGE_ROLE_ACCESS_POLICY_DOCUMENT: &str = r#"{
        "Version": "2012-10-17",
        "Statement": [
            {
                "Effect": "Allow",
                "Action": [
                    "logs:CreateLogGroup",
                    "logs:CreateLogStream",
                    "logs:PutLogEvents",
                    "logs:DescribeLogStreams",
                    "s3:GetBucketLocation"
                ],
                "Resource": "*"
            }
        ]
    }"#;
const IOT_POLICY_DOCUMENT: &str = r#"{
                "Version":"2012-10-17",
                "Statement":[
                    {
                        "Effect":"Allow",
                        "Action":[
                            "iot:Connect",
                            "iot:Publish",
                            "iot:Subscribe",
                            "iot:Receive",
                            "greengrass:*"
                        ],
                        "Resource":"*"
                    }
                ]
                }"#;
pub async fn provision(args: &Args) -> Result<()> {
    let name = &args.thing_name;
    let region = &args.aws_region;
    let policy = &args.thing_policy_name;
    let root = &args.root;
    let group = &args.thing_group_name;
    let role = &args.tes_role_name;
    let role_alias = &args.tes_role_alias_name;

    info!(
        "Provisioning AWS IoT resources for the device with IoT Thing Name: {}",
        name
    );
    createThing(name, region, policy, root).await?;
    info!(
        "Successfully provisioned AWS IoT resources for the device with IoT Thing Name: {}",
        name
    );
    if let Some(group) = group {
        info!("Adding IoT Thing {} into Thing Group: {}...", name, group);
        addThingToGroup(name, group);
        info!("Successfully added Thing into Thing Group: {}", group);
    }

    info!("Setting up resources for {name} ...");
    setupIoTRoleForTes(&role, role_alias, "certificateArn");
    createAndAttachRolePolicy(&role, &region);
    info!("Configuring Nucleus with provisioned resource details...");
    downloadRootCAToFile(Path::new("rootCA.pem")).await;
    info!("Successfully configured Nucleus with provisioned resource details!");
    // if args.deploy_dev_tools {
    //     createInitialDeploymentIfNeeded(group.as_deref(), "cliVersion");
    // }
    Ok(())
}

/**
 * Create a thing with provided configuration.
 *
 * @param client     iotClient to use
 * @param policy policy
 * @param thing_name  thing_name
 * @return created thing info
 */
async fn createThing(
    thing_name: &str,
    region: &str,
    policy: &str,
    root_path: &PathBuf,
) -> Result<()> {
    let region_provider =
        RegionProviderChain::first_try(Region::new(region.to_string())).or_default_provider();
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);
    // Find or create IoT policy
    match client.get_policy().policy_name(policy).send().await {
        Ok => info!("Found IoT policy {}, reusing it", policy),
        Err(_) => {
            info!("Creating new IoT policy {}", policy);
            client
                .create_policy()
                .policy_name(policy)
                .policy_document(IOT_POLICY_DOCUMENT)
                .send()
                .await?;
        }
    }
    // Create cert
    info!("Creating keys and certificate...");
    let keyResponse = client
        .create_keys_and_certificate()
        .set_as_active(true)
        .send()
        .await?;
    fs::write(
        root_path.join("thingCert.crt"),
        &keyResponse
            .certificate_pem()
            .context("Failed to create certificate for thing.")?,
    )?;
    fs::write(
        root_path.join("privKey.key"),
        &keyResponse
            .key_pair
            .as_ref()
            .unwrap()
            .private_key()
            .unwrap(),
    )?;

    let certificate_arn = &keyResponse
        .certificate_arn
        .context("Failed to get certificate arn.")?;
    // Attach policy to cert
    info!("Attaching policy to certificate...");
    let _resp = client
        .attach_policy()
        .policy_name(policy)
        .target(certificate_arn)
        .send()
        .await?;

    // Create the thing and attach the cert to it
    info!("Creating IoT Thing ...");
    let resp = client.create_thing().thing_name(thing_name).send().await?;
    let thing_arn = resp.thing_arn();

    info!("Attaching certificate to IoT thing...");

    let _resp = client
        .attach_thing_principal()
        .thing_name(thing_name)
        .principal(certificate_arn)
        .send()
        .await?;

    let data_endpoint = client
        .describe_endpoint()
        .endpoint_type("iot:Data-ATS")
        .send()
        .await?;
    let cred_endpoint = client
        .describe_endpoint()
        .endpoint_type("iot:CredentialProvider")
        .send()
        .await?;

    Ok(())
}

/**
 * Add an existing Thing into a Thing Group which may or may not exist,
 * creates thing group if it doesn't exist.
 *
 * @param thingName      thing name
 * @param thingGroupName group to add the thing into
 */
fn addThingToGroup(thing_name: &str, thingGroupName: &str) {}
/**
 * Create IoT role for using TES.
 *
 * @param roleName       rolaName
 * @param roleAliasName  roleAlias name
 * @param certificate_arn certificate arn for the IoT thing
 */
pub fn setupIoTRoleForTes(roleName: &str, roleAliasName: &str, certificate_arn: &str) {}

/**
 * Creates IAM policy using specified name and document. Attach the policy to given IAM role name.
 *
 * @param roleName  name of target role
 * @param awsRegion aws region
 * @return ARN of created policy
 */
pub fn createAndAttachRolePolicy(roleName: &str, region: &str) {}

/**
 * Creates an initial deployment to deploy dev tools like the Greengrass CLI component.
 *
 * @param thingInfo thing info for the device
 * @param thingGroupName thing group name
 * @param cliVersion CLI version to install
 */
pub fn createInitialDeploymentIfNeeded(
    // thingInfo: &ThingInfo,
    thingGroupName: Option<&str>,
    cliVersion: &str,
) {
}

const ROOT_CA_URL: &str = "https://www.amazontrust.com/repository/AmazonRootCA1.pem";
/*
 * Download root CA to a local file.
 *
 * To support HTTPS proxies and other custom truststore configurations, append to the file if it exists.
 */
pub async fn downloadRootCAToFile(path: &Path) -> Result<()> {
    if Path::new(path).exists() {
        info!("Root CA file found at . Contents will be preserved.");
    }
    info!(
        "Please download Root CA by curl -o rootCA.pem {}.",
        ROOT_CA_URL
    );
    // info!("Downloading Root CA from {}", ROOT_CA_URL);

    // TODO: append

    // let body = reqwest::get(ROOT_CA_URL).await?.text().await?;

    // debug!("body = {:?}", &body);
    // fs::write(path, body).expect("Unable to write file");

    // downloadFileFromURL(ROOT_CA_URL, path);
    // removeDuplicateCertificates(f);
    // Do not block as the root CA file may have been manually provisioned
    // info!("Failed to download Root CA.");
    Ok(())
}
