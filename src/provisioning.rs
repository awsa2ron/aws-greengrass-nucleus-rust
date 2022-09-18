#![allow(non_snake_case)]
use std::path::{Path, PathBuf};

use anyhow::Ok;
use aws_config::meta::region::RegionProviderChain;
// use greengrassv2 as ggv2;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use tracing::{debug, event, info, span, Level};

#[derive(Serialize, Deserialize, Debug)]
pub struct SystemConfiguration {
    pub certificateFilePath: PathBuf,
    pub privateKeyPath: PathBuf,
    pub rootCaPath: PathBuf,
    pub rootpath: PathBuf,
    pub thingName: String,
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
