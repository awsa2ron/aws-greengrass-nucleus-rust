//! The setup script is intended to give a brand new user of Greengrass to get started with Greengrass device quickly.
//! As part of that experience the user can get a fat bin for the Greengrass Nucleus, the script can launch the Nucleus
//! with the customer's provided config if desired, optionally provision the test device as an AWS IoT Thing, create and
//! attach policies and certificates to it, create TES role and role alias or uses existing ones and attaches
//! them to the IoT thing certificate.
use crate::services;

use super::provisioning;
use anyhow::{Error, Result};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_iot::{model::KeyPair, Client};
use aws_types::region::Region;
use std::fs;
use std::path::Path;
use tracing::{debug, event, info, span, Level};

pub struct ThingInfo {
    thingArn: String,
    thingName: String,
    certificateArn: String,
    certificateId: String,
    certificatePem: String,
    keyPair: KeyPair,
    dataEndpoint: String,
    credEndpoint: String,
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
pub async fn downloadRootCAToFile(path: &Path) -> Result<(), Error> {
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

pub async fn perform_setup(
    client: Client,
    name: &str,
    region: &str,
    need_provisioning: bool,
    thing_policy_name: &str,
) {
    if need_provisioning {
        provision(client, name, thing_policy_name).await;
    }

    info!("Launching Nucleus...");
    services::start_services();
    info!("Launched Nucleus successfully.");
}

async fn provision(client: Client, name: &str, policy_name: &str) {
    info!(
        "Provisioning AWS IoT resources for the device with IoT Thing Name: {}",
        name
    );
    let thing = createThing(client, &name, &policy_name).await.unwrap();
    info!(
        "Successfully provisioned AWS IoT resources for the device with IoT Thing Name: {}",
        name
    );
    info!("Setting up resources for %s ... %n");
    info!("Configuring Nucleus with provisioned resource details...");
    updateKernelConfigWithIotConfiguration(thing).await;
    info!("Successfully configured Nucleus with provisioned resource details!");
}

async fn updateKernelConfigWithIotConfiguration(thing: ThingInfo) {
    // rootDir = kernel.getNucleusPaths().rootPath();
    // let rootDir = Path::new("/greengrass/v2");
    let rootDir = Path::new(".");
    let caFilePath = rootDir.join("rootCA.pem");
    let privKeyFilePath = rootDir.join("privKey.key");
    let certFilePath = rootDir.join("thingCert.crt");

    downloadRootCAToFile(Path::new("rootCA.pem")).await;

    provisioning::updateSystemConfiguration(
        thing.thingName.as_str(),
        caFilePath,
        privKeyFilePath,
        certFilePath,
    );
}

/**
 * Create a thing with provided configuration.
 *
 * @param client     iotClient to use
 * @param policyName policyName
 * @param thingName  thingName
 * @return created thing info
 */
async fn createThing(
    client: Client,
    thingName: &str,
    policyName: &str,
) -> Result<ThingInfo, Error> {
    // Find or create IoT policy
    match client.get_policy().policy_name(policyName).send().await {
        Ok(_) => info!("Found IoT policy {}, reusing it", policyName),
        Err(_) => {
            info!("Creating new IoT policy {}", policyName);
            client
                .create_policy()
                .policy_name(policyName)
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
        &keyResponse.certificate_pem().unwrap_or_default(),
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

    let certificateArn = &keyResponse.certificate_arn.unwrap();
    // Attach policy to cert
    info!("Attaching policy to certificate...");
    let _resp = client
        .attach_policy()
        .policy_name(policyName)
        .target(certificateArn)
        .send()
        .await?;

    // Create the thing and attach the cert to it
    info!("Creating IoT Thing ...");
    let resp = client.create_thing().thing_name(thingName).send().await?;
    let thingArn = resp.thing_arn();

    info!("Attaching certificate to IoT thing...");

    let _resp = client
        .attach_thing_principal()
        .thing_name(thingName)
        .principal(certificateArn)
        .send()
        .await?;

    let dataEndpoint = client
        .describe_endpoint()
        .endpoint_type("iot:Data-ATS")
        .send()
        .await?;
    let credEndpoint = client
        .describe_endpoint()
        .endpoint_type("iot:CredentialProvider")
        .send()
        .await?;

    let thingInfo = ThingInfo {
        thingArn: thingArn.unwrap().to_string(),
        thingName: thingName.to_string(),
        certificateArn: certificateArn.to_string(),
        certificateId: certificateArn.to_string(),
        certificatePem: certificateArn.to_string(),
        keyPair: keyResponse.key_pair.unwrap(),
        dataEndpoint: dataEndpoint.endpoint_address.unwrap(),
        credEndpoint: credEndpoint.endpoint_address.unwrap(),
    };
    Ok(thingInfo)
}

// /**
//  * Create IoT role for using TES.
//  *
//  * @param roleName       rolaName
//  * @param roleAliasName  roleAlias name
//  * @param certificateArn certificate arn for the IoT thing
//  */
// pub fn setupIoTRoleForTes(roleName: String, roleAliasName: String, certificateArn: String) {
//     // String roleAliasArn;
//     // try {
//         // Get Role Alias arn
//         DescribeRoleAliasRequest describeRoleAliasRequest =
//                 DescribeRoleAliasRequest.builder().roleAlias(roleAliasName).build();
//         roleAliasArn = iotClient.describeRoleAlias(describeRoleAliasRequest).roleAliasDescription().roleAliasArn();
//     // } catch (ResourceNotFoundException ranfe) {
//         info!("TES role alias \"%s\" does not exist, creating new alias...%n", roleAliasName);

//         // Get IAM role arn in order to attach an alias to it
//         String roleArn;
//         try {
//             GetRoleRequest getRoleRequest = GetRoleRequest.builder().roleName(roleName).build();
//             roleArn = iamClient.getRole(getRoleRequest).role().arn();
//         } catch (NoSuchEntityException | ResourceNotFoundException rnfe) {
//             info!("TES role \"%s\" does not exist, creating role...%n", roleName);
//             CreateRoleRequest createRoleRequest = CreateRoleRequest.builder().roleName(roleName).description(
//                     "Role for Greengrass IoT things to interact with AWS services using token exchange service")
//                     .assumeRolePolicyDocument("{\n  \"Version\": \"2012-10-17\",\n"
//                             + "  \"Statement\": [\n    {\n      \"Effect\": \"Allow\",\n"
//                             + "      \"Principal\": {\n       \"Service\": \"" + tesServiceEndpoints.get(envStage)
//                             + "\"\n      },\n      \"Action\": \"sts:AssumeRole\"\n    }\n  ]\n}").build();
//             roleArn = iamClient.createRole(createRoleRequest).role().arn();
//         }

//         CreateRoleAliasRequest createRoleAliasRequest =
//                 CreateRoleAliasRequest.builder().roleArn(roleArn).roleAlias(roleAliasName).build();
//         roleAliasArn = iotClient.createRoleAlias(createRoleAliasRequest).roleAliasArn();
//     // }

//     // Attach policy role alias to cert
//     String iotRolePolicyName = IOT_ROLE_POLICY_NAME_PREFIX + roleAliasName;
//     try {
//         iotClient.getPolicy(GetPolicyRequest.builder().policyName(iotRolePolicyName).build());
//     } catch (ResourceNotFoundException e) {
//         info!("IoT role policy \"%s\" for TES Role alias not exist, creating policy...%n",
//                 iotRolePolicyName);
//         CreatePolicyRequest createPolicyRequest = CreatePolicyRequest.builder().policyName(iotRolePolicyName)
//                 .policyDocument("{\n\t\"Version\": \"2012-10-17\",\n\t\"Statement\": {\n"
//                         + "\t\t\"Effect\": \"Allow\",\n\t\t\"Action\": \"iot:AssumeRoleWithCertificate\",\n"
//                         + "\t\t\"Resource\": \"" + roleAliasArn + "\"\n\t}\n}").build();
//         iotClient.createPolicy(createPolicyRequest);
//     }

//     outStream.println("Attaching TES role policy to IoT thing...");
//     AttachPolicyRequest attachPolicyRequest =
//             AttachPolicyRequest.builder().policyName(iotRolePolicyName).target(certificateArn).build();
//     iotClient.attachPolicy(attachPolicyRequest);
// }
