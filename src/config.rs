use once_cell::sync::OnceCell;
use serde::Deserialize;
use serde_yaml::{Value};
use std::fs;
use std::path::Path;

use crate::provisioning;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub system: provisioning::SystemConfiguration,
    pub services: Services,
}

pub static CONFIG: OnceCell<Config> = OnceCell::new();

impl Config {
    pub fn global() -> &'static Config {
        CONFIG.get().expect("config is not initialized")
    }

    fn from_config_file(path: &Path) -> Result<Config, std::io::Error> {
        let config = fs::read_to_string(path).expect("Something went wrong reading config file");
        let config: Config =
            serde_yaml::from_str(&config).expect("Something went wrong deserializing config file");
        Ok(config)
    }
}

pub fn init(path: &Path) {
    let mut _config =
        Config::from_config_file(path).expect("Something went wrong reading config file");

    CONFIG.set(_config).unwrap();
}

#[derive(Deserialize, Debug)]
pub struct Services {
    #[serde(rename = "aws.greengrass.Nucleus")]
    pub kernel: Kernel,
}
#[derive(Deserialize, Debug)]
pub struct Kernel {
    pub componentType: String,
    pub configuration: Configuration,
    pub dependencies: Value,
    pub version: String,
}
#[derive(Deserialize, Debug)]
pub struct Configuration {
    pub awsRegion: String,
    pub greengrassDataPlaneEndpoint: String,
    pub iotCredEndpoint: String,
    pub iotDataEndpoint: String,
    pub iotRoleAlias: String,
    pub runWithDefault: Value,
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn global_config_test() {
//         use super::{init, Config};
//         init("./config/config.yaml");
//     }
// }
