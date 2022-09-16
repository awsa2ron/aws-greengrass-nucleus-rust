#![allow(non_snake_case)]
use std::path::{Path, PathBuf};

use anyhow::Ok;
use aws_config::meta::region::RegionProviderChain;
// use greengrassv2 as ggv2;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use tracing::{debug, event, info, span, Level};

#[derive(Deserialize, Debug)]
pub struct SystemConfiguration {
    pub certificateFilePath: PathBuf,
    pub privateKeyPath: PathBuf,
    pub rootCaPath: PathBuf,
    pub rootpath: PathBuf,
    thingName: String,
}

pub static SYSCONFIG: OnceCell<SystemConfiguration> = OnceCell::new();

impl SystemConfiguration {
    pub fn global() -> &'static SystemConfiguration {
        SYSCONFIG
            .get()
            .expect("System configuration is not initialized")
    }
    fn update(
        thingName: String,
        certificateFilePath: PathBuf,
        privateKeyPath: PathBuf,
        rootCaPath: PathBuf,
        rootpath: PathBuf,
    ) -> Result<SystemConfiguration, anyhow::Error> {
        let ret = SystemConfiguration {
            certificateFilePath,
            privateKeyPath,
            rootCaPath,
            rootpath,
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
    thingName: &str,
    caFilePath: PathBuf,
    privKeyFilePath: PathBuf,
    certFilePath: PathBuf,
    rootpath: PathBuf,
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
        thingName.to_string(),
        certFilePath,
        privKeyFilePath,
        caFilePath,
        rootpath,
    )
    .unwrap();
    SYSCONFIG.set(sysConfig).unwrap();
}

#[derive(Debug)]
pub struct NucleusConfiguration {
    awsRegion: String,
    iotCredentialsEndpoint: String,
    iotDataEndpoint: String,
    iotRoleAlias: String,
}
pub static NUCLEUSCONFIG: OnceCell<NucleusConfiguration> = OnceCell::new();

impl NucleusConfiguration {
    pub fn global() -> &'static NucleusConfiguration {
        NUCLEUSCONFIG
            .get()
            .expect("System configuration is not initialized")
    }

    fn update(
        awsRegion: String,
        iotCredentialsEndpoint: String,
        iotDataEndpoint: String,
        iotRoleAlias: String,
    ) -> Result<NucleusConfiguration, anyhow::Error> {
        let ret = NucleusConfiguration {
            awsRegion,
            iotCredentialsEndpoint,
            iotDataEndpoint,
            iotRoleAlias,
        };
        Ok(ret)
    }
}
/**
 * Updates the nucleus configuration value in kernel config as per the given {@link NucleusConfiguration}.
 * @param nucleusConfiguration {@link NucleusConfiguration}
 * @param updateBehavior Update behavior indicating either merge or replace
 */
pub fn updateNucleusConfiguration(
    awsRegion: String,
    iotCredentialsEndpoint: String,
    iotDataEndpoint: String,
    iotRoleAlias: String,
) {
    // Map<String, Object> updateMap = new HashMap<>();
    // if (nucleusConfiguration.getAwsRegion() != null) {
    //     updateMap.put(DeviceConfiguration.DEVICE_PARAM_AWS_REGION, nucleusConfiguration.getAwsRegion());
    // }
    // if (nucleusConfiguration.getIotCredentialsEndpoint() != null) {
    //     updateMap.put(DeviceConfiguration.DEVICE_PARAM_IOT_CRED_ENDPOINT, nucleusConfiguration
    //             .getIotCredentialsEndpoint());
    // }
    // if (nucleusConfiguration.getIotDataEndpoint() != null) {
    //     updateMap.put(DeviceConfiguration.DEVICE_PARAM_IOT_DATA_ENDPOINT, nucleusConfiguration
    //             .getIotDataEndpoint());
    // }
    // if (nucleusConfiguration.getIotRoleAlias() != null) {
    //     updateMap.put(DeviceConfiguration.IOT_ROLE_ALIAS_TOPIC, nucleusConfiguration.getIotRoleAlias());
    // }
    // String nucleusComponentName = kernel.getContext().get(DeviceConfiguration.class).getNucleusComponentName();
    // Topics nucleusConfig = kernel.getConfig()
    //         .lookupTopics(SERVICES_NAMESPACE_TOPIC, nucleusComponentName, CONFIGURATION_CONFIG_KEY);
    // nucleusConfig.updateFromMap(updateMap,  new UpdateBehaviorTree(updateBehavior, System.currentTimeMillis()));
    let nucleusConfig = NucleusConfiguration::update(
        awsRegion.to_string(),
        iotCredentialsEndpoint,
        iotDataEndpoint,
        iotRoleAlias,
    )
    .unwrap();
    NUCLEUSCONFIG.set(nucleusConfig).unwrap();
}
pub struct ProvisionConfiguration {
    systemConfiguration: OnceCell<SystemConfiguration>,
    nucleusConfiguration: OnceCell<NucleusConfiguration>,
}

impl ProvisionConfiguration {}
