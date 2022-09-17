use once_cell::sync::OnceCell;
use serde::Deserialize;
use serde_yaml::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::provisioning;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub system: provisioning::SystemConfiguration,
    pub services: Services,
}
const CONFIG_FILE_PATH: &'static str =  "config/config.yaml";


pub static CONFIG: OnceCell<Config> = OnceCell::new();

impl Config {
    pub fn global() -> &'static Config {
        CONFIG.get().expect("config is not initialized")
    }

    fn from_config_file(path: &Path) -> Result<Config, std::io::Error> {
        println!("{path:#?}");
        let config = fs::read_to_string(path).expect("Something went wrong reading config file");
        let config: Config =
            serde_yaml::from_str(&config).expect("Something went wrong deserializing config file");
        Ok(config)
    }
}

pub fn init(path_args: &Option<PathBuf>) {
    let path = match path_args {
        Some(path) => path.to_owned(),
        None => PathBuf::from_str(CONFIG_FILE_PATH).unwrap(),
    };

    let mut _config =
        Config::from_config_file(&path).expect("Something went wrong reading config file");

    CONFIG.set(_config).unwrap();
}

#[derive(Deserialize, Debug)]
pub struct Services {
    #[serde(rename = "aws.greengrass.Nucleus")]
    pub kernel: Kernel,
}
#[derive(Deserialize, Debug)]
pub struct Kernel {
    #[serde(rename = "componentType")]
    pub component: String,
    pub configuration: Configuration,
    pub dependencies: Value,
    pub version: String,
}
#[derive(Deserialize, Debug)]
pub struct Configuration {
    #[serde(rename = "awsRegion")]
    pub region: String,
    #[serde(rename = "greengrassDataPlaneEndpoint")]
    pub gg_data_plane_endpoint: String,
    #[serde(rename = "iotCredEndpoint")]
    pub iot_cred_endpoint: String,
    #[serde(rename = "iotDataEndpoint")]
    pub iot_data_endpoint: String,
    #[serde(rename = "iotRoleAlias")]
    pub iot_role_alias: String,
    #[serde(rename = "runWithDefault")]
    pub run_with_default: Value,
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn global_config_test() {
//         use super::{init, Config};
//         init("./config/config.yaml");
//     }
// }
