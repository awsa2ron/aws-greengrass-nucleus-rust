use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::fs;
use std::path::Path;

use crate::provisioning;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub id: String,
    pub endpoint: Endpoint,
    system: provisioning::SystemConfiguration,
    // pub certificates: Certificates,
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
#[derive(Deserialize, Debug)]
pub struct Endpoint {
    pub iot: String,
    pub iot_ats: String,
    // pub credential: String,
    // pub jobs: String,
}
#[derive(Deserialize, Debug)]
pub struct Certificates {
    pub ca: String,
    pub cert: String,
    pub key: String,
}

pub fn init(path: &Path) {
    let mut _config =
        Config::from_config_file(path).expect("Something went wrong reading config file");

    CONFIG.set(_config).unwrap();
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn global_config_test() {
//         use super::{init, Config};
//         init("./config/config.yaml");
//     }
// }
