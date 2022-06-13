pub mod greengrassv2;
pub mod iot;

use std::path::{Path, PathBuf};

use anyhow::Ok;
use aws_config::meta::region::RegionProviderChain;
// use greengrassv2 as ggv2;
use once_cell::sync::OnceCell;
use tracing::{debug, event, info, span, Level};

#[derive(Debug)]
pub struct SystemConfiguration {
    pub certificateFilePath: PathBuf,
    pub privateKeyPath: PathBuf,
    pub rootCAPath: PathBuf,
    thingName: String,
}

pub static SYSCONFIG: OnceCell<SystemConfiguration> = OnceCell::new();

impl SystemConfiguration {
    /**
     * Updates the system configuration values in kernel config as per the given {@link SystemConfiguration}.
     * @param systemConfiguration {@link SystemConfiguration}
     * @param updateBehavior Update behavior indicating either merge or replace
     */
    pub fn global() -> &'static SystemConfiguration {
        SYSCONFIG
            .get()
            .expect("System configuration is not initialized")
    }

    fn update(
        thingName: String,
        certificateFilePath: PathBuf,
        privateKeyPath: PathBuf,
        rootCAPath: PathBuf,
    ) -> Result<SystemConfiguration, anyhow::Error> {
        let ret: SystemConfiguration = SystemConfiguration {
            certificateFilePath,
            privateKeyPath,
            rootCAPath,
            thingName,
        };
        Ok(ret)
    }
}

/**
 * Updates the system configuration values in kernel config as per the given {@link SystemConfiguration}.
 * @param systemConfiguration {@link SystemConfiguration}
 * @param updateBehavior Update behavior indicating either merge or replace
 */
pub fn updateSystemConfiguration(
    thing_name: &str,
    caFilePath: PathBuf,
    privKeyFilePath: PathBuf,
    certFilePath: PathBuf,
) {
    //   @NonNull UpdateBehaviorTree.UpdateBehavior updateBehavior) {
    // Map<String, Object> updateMap = new HashMap<>();
    // if (systemConfiguration.getCertificateFilePath() != null) {
    //     updateMap.put(DeviceConfiguration.DEVICE_PARAM_CERTIFICATE_FILE_PATH,
    //             systemConfiguration.getCertificateFilePath());
    // }
    // if (systemConfiguration.getPrivateKeyPath() != null) {
    //     updateMap.put(DeviceConfiguration.DEVICE_PARAM_PRIVATE_KEY_PATH, systemConfiguration.getPrivateKeyPath());
    // }
    // if (systemConfiguration.getThingName() != null) {
    //     updateMap.put(DeviceConfiguration.DEVICE_PARAM_THING_NAME, systemConfiguration.getThingName());
    // }
    // if (systemConfiguration.getRootCAPath() != null) {
    //     updateMap.put(DeviceConfiguration.DEVICE_PARAM_ROOT_CA_PATH, systemConfiguration.getRootCAPath());
    // }
    // Topics systemConfig = kernel.getConfig().lookupTopics(SYSTEM_NAMESPACE_KEY);
    // systemConfig.updateFromMap(updateMap, new UpdateBehaviorTree(updateBehavior, System.currentTimeMillis()));
    let sysConfig = SystemConfiguration::update(
        thing_name.to_string(),
        caFilePath,
        privKeyFilePath,
        certFilePath,
    )
    .unwrap();
    SYSCONFIG.set(sysConfig).unwrap();
}

//  struct NucleusConfiguration {
//     awsRegion: String,
//     iotCredentialsEndpoint: String,
//     iotDataEndpoint: String,
//     iotRoleAlias: String,
// }

// impl NucleusConfiguration {
//     /**
//      * Updates the nucleus configuration value in kernel config as per the given {@link NucleusConfiguration}.
//      * @param nucleusConfiguration {@link NucleusConfiguration}
//      * @param updateBehavior Update behavior indicating either merge or replace
//      */
//     pub fn updateNucleusConfiguration() {}
// }

pub struct ProvisionConfiguration {
    systemConfiguration: OnceCell<SystemConfiguration>,
    // nucleusConfiguration: OnceCell<NucleusConfiguration>,
}

impl ProvisionConfiguration {}

// pub class ProvisionContext {
//     String provisioningPolicy;
//     Map<String, Object> parameterMap;
// }

pub async fn init(region: RegionProviderChain) {
    // let shared_config = aws_config::from_env().region(region).load().await;

    // ggv2::ggv2_init(&shared_config).await;
}

// const FLOW: String = r#"Provisioning AWS IoT resources for the device with IoT Thing Name: [GreengrassQuickStartCore-new]... -> describe-endpoint
//                         Found IoT policy "GreengrassV2IoTThingPolicy", reusing it -> list-policies | create-policy & get-policy
//                         Creating keys and certificate... -> create-keys-and-certificate
//                         Attaching policy to certificate...  -> attach-policy
//                         Creating IoT Thing "GreengrassQuickStartCore-new"... -> create-thing
//                         Attaching certificate to IoT thing... -> attach-thing-principal
//                         Successfully provisioned AWS IoT resources for the device with IoT Thing Name: [GreengrassQuickStartCore-new]!
//                         Setting up resources for aws.greengrass.TokenExchangeService ... -> IAM create-role
//                         Attaching TES role policy to IoT thing... -> IAM attach-role-policy
//                         No managed IAM policy found, looking for user defined policy... -> IAM create-policy
//                         IAM policy named "GreengrassV2TokenExchangeRoleAccess" already exists. Please attach it to the IAM role if not already
//                         Configuring Nucleus with provisioned resource details... -> create-role-alias
//                         Root CA file found at "/greengrass/v2/rootCA.pem". Contents will be preserved. -> reqwest crate.
//                         Downloading Root CA from "https://www.amazontrust.com/repository/AmazonRootCA1.pem"
//                         Created device configuration -> config file.
//                         Successfully configured Nucleus with provisioned resource details!
//                         Launching Nucleus... -> mqtt publish.
//                         Launched Nucleus successfully.
//                         "#;
