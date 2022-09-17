#![allow(non_snake_case)]
//! The setup script is intended to give a brand new user of Greengrass to get started with Greengrass device quickly.
//! As part of that experience the user can get a fat bin for the Greengrass Nucleus, the script can launch the Nucleus
//! with the customer's provided config if desired, optionally provision the test device as an AWS IoT Thing, create and
//! attach policies and certificates to it, create TES role and role alias or uses existing ones and attaches
//! them to the IoT thing certificate.
use crate::{services, Args};

use super::provisioning;
use anyhow::{Context, Error, Ok, Result};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_iot::{model::KeyPair, Client};
use aws_types::region::Region;
use rumqttc::{AsyncClient, ClientError, QoS};
use serde_json::json;
use std::path::Path;
use std::{fs, path::PathBuf};
use tracing::{debug, event, info, span, Level};

pub struct ThingInfo {
    thing_arn: String,
    thing_name: String,
    certificate_arn: String,
    certificate_id: String,
    certificate_pem: String,
    key_pair: KeyPair,
    data_endpoint: String,
    cred_endpoint: String,
}

const GG_TOKEN_EXCHANGE_ROLE_ACCESS_POLICY_SUFFIX: &str = "Access";
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
const ROOT_CA_URL: &str = "https://www.amazontrust.com/repository/AmazonRootCA1.pem";
const IOT_ROLE_POLICY_NAME_PREFIX: &str = "GreengrassTESCertificatePolicy";
const GREENGRASS_CLI_COMPONENT_NAME: &str = "aws.greengrass.Cli";
const INITIAL_DEPLOYMENT_NAME_FORMAT: &str = "Deployment for %s";
const IAM_POLICY_ARN_FORMAT: &str = "arn:%s:iam::%s:policy/%s";
const MANAGED_IAM_POLICY_ARN_FORMAT: &str = "arn:%s:iam::aws:policy/%s";

const E2E_TESTS_POLICY_NAME_PREFIX: &str = "E2ETestsIotPolicy";
const E2E_TESTS_THING_NAME_PREFIX: &str = "E2ETestsIotThing";

// final Map<EnvironmentStage, String> tesServiceEndpoints = ImmutableMap.of(
//         EnvironmentStage.PROD, "credentials.iot.amazonaws.com",
//         EnvironmentStage.GAMMA, "credentials.iot.test.amazonaws.com",
//         EnvironmentStage.BETA, "credentials.iot.test.amazonaws.com"
// );

/*
 * Download root CA to a local file.
 *
 * To support HTTPS proxies and other custom truststore configurations, append to the file if it exists.
 */
pub async fn downloadRootCAToFile(path: &Path) -> Result<()> {
    if Path::new(path).exists() {
        info!("Root CA file found at . Contents will be preserved.");
    }
    info!("Downloading Root CA from {}", ROOT_CA_URL);

    // TODO: append

    let body = reqwest::get(ROOT_CA_URL).await?.text().await?;

    debug!("body = {:?}", &body);
    fs::write(path, body).expect("Unable to write file");

    // downloadFileFromURL(ROOT_CA_URL, path);
    // removeDuplicateCertificates(f);
    // Do not block as the root CA file may have been manually provisioned
    // info!("Failed to download Root CA.");
    Ok(())
}

pub async fn provision(args: &Args) -> Result<()> {
    let name = &args.thing_name;
    let region = &args.aws_region;
    let policy = &args.thing_policy_name;
    let group = &args.thing_group_name;
    let role = &args.tes_role_name;
    let role_alias = &args.tes_role_alias_name;
    let dev = args.deploy_dev_tools;

    info!(
        "Provisioning AWS IoT resources for the device with IoT Thing Name: {}",
        name
    );
    let thing = createThing(name, region, policy).await?;
    info!(
        "Successfully provisioned AWS IoT resources for the device with IoT Thing Name: {}",
        name
    );
    if let Some(group) = group {
        info!("Adding IoT Thing {} into Thing Group: {}...", name, name);
        addThingToGroup(name, name);
        info!("Successfully added Thing into Thing Group: {}", name);
    }

    info!("Setting up resources for %s ... %n");
    setupIoTRoleForTes(&role, role_alias, "certificateArn");
    createAndAttachRolePolicy(&role, &region);
    info!("Configuring Nucleus with provisioned resource details...");
    updateKernelConfigWithIotConfiguration(&thing).await;
    info!("Successfully configured Nucleus with provisioned resource details!");
    if dev {
        createInitialDeploymentIfNeeded(&thing, group.as_deref(), "cliVersion");
    }
    Ok(())
}

async fn updateKernelConfigWithIotConfiguration(thing: &ThingInfo) {
    // rootDir = kernel.getNucleusPaths().rootPath();
    // let rootDir = Path::new("/greengrass/v2");
    let rootDir = PathBuf::new();
    let caFilePath = rootDir.join("rootCA.pem");
    let privKeyFilePath = rootDir.join("privKey.key");
    let certFilePath = rootDir.join("thingCert.crt");

    downloadRootCAToFile(Path::new("rootCA.pem")).await;

    provisioning::updateSystemConfiguration(
        thing.thing_name.as_str(),
        caFilePath,
        privKeyFilePath,
        certFilePath,
        rootDir,
    );
}

/**
 * Create a thing with provided configuration.
 *
 * @param client     iotClient to use
 * @param policy policy
 * @param thing_name  thing_name
 * @return created thing info
 */
async fn createThing(thing_name: &str, region: &str, policy: &str) -> Result<ThingInfo> {
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
    let rootDir = Path::new(".");
    fs::write(
        rootDir.join("thingCert.crt"),
        &keyResponse
            .certificate_pem()
            .context("Failed to create certificate for thing.")?,
    )?;
    fs::write(
        rootDir.join("privKey.key"),
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

    let thingInfo = ThingInfo {
        thing_arn: thing_arn.unwrap().to_string(),
        thing_name: thing_name.to_string(),
        certificate_arn: certificate_arn.to_string(),
        certificate_id: certificate_arn.to_string(),
        certificate_pem: certificate_arn.to_string(),
        key_pair: keyResponse.key_pair.unwrap(),
        data_endpoint: data_endpoint.endpoint_address.unwrap(),
        cred_endpoint: cred_endpoint.endpoint_address.unwrap(),
    };
    Ok(thingInfo)
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
    thingInfo: &ThingInfo,
    thingGroupName: Option<&str>,
    cliVersion: &str,
) {
}
