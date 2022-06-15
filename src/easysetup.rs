use super::provisioning;
// use anyhow::Result;
use anyhow::{Error, Result};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_iot::Client;
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
    // keyPair: String,
    // dataEndpoint: String,
    // credEndpoint: String,
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

fn downloadFileFromURL(url: &str, path: &Path) {
    // String certificates = new String(Files.readAllBytes(f.toPath()), StandardCharsets.UTF_8);
    // Set<String> uniqueCertificates =
    //         Arrays.stream(certificates.split(EncryptionUtils.CERTIFICATE_PEM_HEADER))
    //                 .map(s -> s.trim())
    //                 .collect(Collectors.toSet());

    // try (BufferedWriter bw = Files.newBufferedWriter(f.toPath(), StandardCharsets.UTF_8)) {
    //     for (String certificate : uniqueCertificates) {
    //         if (certificate.length() > 0) {
    //             bw.write(EncryptionUtils.CERTIFICATE_PEM_HEADER);
    //             bw.write("");
    //             bw.write(certificate);
    //             bw.write("");
    //         }
    //     }
    // }
    info!("Failed to remove duplicate certificates - %s%n");
}
// fn removeDuplicateCertificates() {
//     SdkHttpFullRequest request = SdkHttpFullRequest.builder()
//                 .uri(URI.create(url))
//                 .method(SdkHttpMethod.GET)
//                 .build();

//         HttpExecuteRequest executeRequest = HttpExecuteRequest.builder()
//                 .request(request)
//                 .build();

//         try (SdkHttpClient client = getSdkHttpClient()) {
//             HttpExecuteResponse executeResponse = client.prepareRequest(executeRequest).call();

//             int responseCode = executeResponse.httpResponse().statusCode();
//             if (responseCode != HttpURLConnection.HTTP_OK) {
//                 throw new IOException("Received invalid response code: " + responseCode);
//             }

//             try (InputStream inputStream = executeResponse.responseBody().get();
//                  OutputStream outputStream = Files.newOutputStream(f.toPath(), StandardOpenOption.CREATE,
//                          StandardOpenOption.APPEND)) {
//                 IoUtils.copy(inputStream, outputStream);
//             }
//         }}

pub async fn performSetup(
    name: String,
    region: String,
    needProvisioning: bool,
    thing_policy_name: Option<String>,
) {
    let region_provider = RegionProviderChain::first_try(Region::new(region))
        .or_default_provider()
        .or_else(Region::new("us_west_2"));
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    // // Describe usage of the command
    // if (showHelp) {
    //     info!(SHOW_HELP_RESPONSE);
    //     return;
    // }
    // if (showVersion) {
    //     // Use getVersionFromBuildMetadataFile so that we don't need to startup the Nucleus which is slow and will
    //     // start creating files and directories which may not be desired
    //     info!(String.format(SHOW_VERSION_RESPONSE,
    //             DeviceConfiguration.getVersionFromBuildRecipeFile()));
    //     return;
    // }

    // if (kernel == null) {
    //     kernel = new Kernel();
    // }
    // kernel.parseArgs(kernelArgs.toArray(new String[]{}));

    // try {
    //     IotSdkClientFactory.EnvironmentStage.fromString(environmentStage);
    // } catch (InvalidEnvironmentStageException e) {
    //     throw new RuntimeException(e);
    // }

    // if (!Utils.isEmpty(trustedPluginPaths)) {
    //     copyTrustedPlugins(kernel, trustedPluginPaths);
    // }
    // DeviceConfiguration deviceConfiguration = kernel.getContext().get(DeviceConfiguration.class);
    if needProvisioning {
        // if (Utils.isEmpty(awsRegion)) {
        //     awsRegion = Coerce.toString(deviceConfiguration.getAWSRegion());
        // }

        // if (Utils.isEmpty(awsRegion)) {
        //     throw new RuntimeException("Required input aws region not provided for provisioning");
        // }

        // this.deviceProvisioningHelper = new DeviceProvisioningHelper(awsRegion, environmentStage, this.outStream);
        // provision(kernel);
        provision(client, name, thing_policy_name.unwrap()).await;
    }

    // // Attempt this only after config file and Nucleus args have been parsed
    // setComponentDefaultUserAndGroup(deviceConfiguration);

    // if (setupSystemService) {
    //     kernel.getContext().get(KernelLifecycle.class).softShutdown(30);
    //     boolean ok = kernel.getContext().get(SystemServiceUtilsFactory.class).getInstance()
    //             .setupSystemService(kernel.getContext().get(KernelAlternatives.class));
    //     if (ok) {
    //         info!("Successfully set up Nucleus as a system service");
    //         // Nucleus will be launched by OS as a service
    //     } else {
    //         info!("Unable to set up Nucleus as a system service");
    //     }
    //     kernel.shutdown();
    //     return;
    // }
    // if (!kernelStart) {
    //     info!("Nucleus start set to false, exiting...");
    //     kernel.shutdown();
    //     return;
    // }
    info!("Launching Nucleus...");
    // kernel.launch();
    info!("Launched Nucleus successfully.");
}

async fn provision(client: Client, name: String, policy_name: String) {
    info!(
        "Provisioning AWS IoT resources for the device with IoT Thing Name: {}",
        name
    );
    let thing = createThing(client, &name, &policy_name).await.unwrap();
    info!(
        "Successfully provisioned AWS IoT resources for the device with IoT Thing Name: {}",
        name
    );
    // if (!Utils.isEmpty(thingGroupName)) {
    //     info!("Adding IoT Thing [%s] into Thing Group: [%s]...%n", thingName, thingGroupName);
    //     deviceProvisioningHelper
    //             .addThingToGroup(deviceProvisioningHelper.getIotClient(), thingName, thingGroupName);
    //     info!("Successfully added Thing into Thing Group: [%s]%n", thingGroupName);
    // }
    // info!("Setting up resources for %s ... %n", TokenExchangeService.TOKEN_EXCHANGE_SERVICE_TOPICS);
    info!("Setting up resources for %s ... %n");
    // deviceProvisioningHelper.setupIoTRoleForTes(tesRoleName, tesRoleAliasName, thingInfo.getCertificateArn());
    // deviceProvisioningHelper.createAndAttachRolePolicy(tesRoleName, Region.of(awsRegion));
    info!("Configuring Nucleus with provisioned resource details...");
    // deviceProvisioningHelper.updateKernelConfigWithIotConfiguration(kernel, thingInfo, awsRegion, tesRoleAliasName);
    updateKernelConfigWithIotConfiguration(thing);
    info!("Successfully configured Nucleus with provisioned resource details!");
    // if (deployDevTools) {
    //     deviceProvisioningHelper.createInitialDeploymentIfNeeded(thingInfo, thingGroupName,
    //             kernel.getContext().get(DeviceConfiguration.class).getNucleusVersion());
    // }

    // // Dump config since we've just provisioned so that the bootstrap config will enable us to
    // // reach the cloud when needed. Must do this now because we normally would never overwrite the bootstrap
    // // file, however we need to do it since we've only just learned about our endpoints, certs, etc.
    // kernel.writeEffectiveConfigAsTransactionLog(kernel.getNucleusPaths().configPath()
    //         .resolve(Kernel.DEFAULT_BOOTSTRAP_CONFIG_TLOG_FILE));
}

async fn updateKernelConfigWithIotConfiguration(thing: ThingInfo) {
    // rootDir = kernel.getNucleusPaths().rootPath();
    // let rootDir = Path::new("/greengrass/v2");
    let rootDir = Path::new(".");
    let caFilePath = rootDir.join("rootCA.pem");
    let privKeyFilePath = rootDir.join("privKey.key");
    let certFilePath = rootDir.join("thingCert.crt");

    downloadRootCAToFile(Path::new("rootCA.pem")).await;

    // try (CommitableFile cf = CommitableFile.of(privKeyFilePath, true)) {
    //     cf.write(thing.keyPair.privateKey().getBytes(StandardCharsets.UTF_8));
    // }
    // try (CommitableFile cf = CommitableFile.of(certFilePath, true)) {
    //     cf.write(thing.certificatePem.getBytes(StandardCharsets.UTF_8));
    // }

    provisioning::updateSystemConfiguration(
        thing.thingName.as_str(),
        caFilePath,
        privKeyFilePath,
        certFilePath,
    );
    // provisioning::updateNucleusConfiguration(
    //     awsRegion,
    //     thing.dataEndpoint,
    //     thing.credEndpoint,
    //     roleAliasName,
    // );
    // new DeviceConfiguration(kernel, thing.thingName, thing.dataEndpoint, thing.credEndpoint,
    //         privKeyFilePath.toString(), certFilePath.toString(), caFilePath.toString(), awsRegion, roleAliasName);
    // // Make sure tlog persists the device configuration
    // kernel.getContext().waitForPublishQueueToClear();
    // info!("Created device configuration");
}
async fn createThing(
    client: Client,
    thingName: &str,
    policyName: &str,
) -> Result<ThingInfo, Error> {
    // Find or create IoT policy
    match client.get_policy().policy_name(thingName).send().await {
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
    // let certificate_pem_outfile = &provisioning::SystemConfiguration::global().certificateFilePath;
    // let public_key_outfile = &provisioning::SystemConfiguration::global().privateKeyPath;
    // let private_key_outfile = &provisioning::SystemConfiguration::global().privateKeyPath;

    let rootDir = Path::new(".");
    // let caFilePath = rootDir.join("rootCA.pem");
    let private_key_outfile = rootDir.join("privKey.key");
    let certificate_pem_outfile = rootDir.join("thingCert.crt");

    let keyResponse = client
        .create_keys_and_certificate()
        .set_as_active(true)
        .send()
        .await?;

    fs::write(
        certificate_pem_outfile,
        &keyResponse.certificate_pem().unwrap_or_default(),
    )?;
    // fs::write(public_key_outfile, resp.key_pair.unwrap().public_key().unwrap())?;
    fs::write(
        private_key_outfile,
        &keyResponse.key_pair.unwrap().private_key().unwrap(),
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
    info!("Creating IoT Thing ...%n");
    let resp = client.create_thing().thing_name(thingName).send().await?;
    let thingArn = resp.thing_arn();

    info!("Attaching certificate to IoT thing...");

    let _resp = client
        .attach_thing_principal()
        .thing_name(thingName)
        .principal(certificateArn)
        .send()
        .await?;

    let thingInfo = ThingInfo {
        thingArn: thingArn.unwrap().to_string(),
        thingName: thingName.to_string(),
        // certificateArn: keyResponse.certificate_arn().unwrap().to_string(),
        certificateArn: certificateArn.to_string(),
        certificateId: certificateArn.to_string(),
        certificatePem: certificateArn.to_string(),
        // certificateId: keyResponse.certificate_id().unwrap().to_string(),

        // certificatePem: keyResponse.certificate_pem().unwrap().to_string(),
    };
    Ok(thingInfo)
    // , keyResponse.keyPair())
    //         client.describeEndpoint(DescribeEndpointRequest.builder().endpointType("iot:Data-ATS").build())
    //                 .endpointAddress(), client.describeEndpoint(
    //         DescribeEndpointRequest.builder().endpointType("iot:CredentialProvider").build()).endpointAddress());

    // ThingInfo()
}
