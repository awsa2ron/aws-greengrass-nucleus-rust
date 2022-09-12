use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::fs;

const CONFIG_FILE: &str = "./config/config.yaml";

#[derive(Deserialize, Debug)]
pub struct Config {
    pub id: String,
    pub endpoint: Endpoint,
    // pub certificates: Certificates,
}

pub static CONFIG: OnceCell<Config> = OnceCell::new();

impl Config {
    pub fn global() -> &'static Config {
        CONFIG.get().expect("logger is not initialized")
    }

    fn from_config_file() -> Result<Config, std::io::Error> {
        let config =
            fs::read_to_string(CONFIG_FILE).expect("Something went wrong reading config file");
        let config: Config = serde_yaml::from_str(&config).expect("Something went wrong deserializing config file");
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

pub fn init() {
    let mut _config = Config::from_config_file().unwrap();

    CONFIG.set(_config).unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn global_config_test() {
        use super::{init, Config};
        init();
    }
}
