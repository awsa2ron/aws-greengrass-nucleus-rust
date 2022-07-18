use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::fs;

const CONFIG_FILE: &str = "config.toml";

#[derive(Deserialize, Debug)]
pub struct Config {
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
            fs::read_to_string(CONFIG_FILE).expect("Something went wrong reading the file");
        let config: Config = toml::from_str(&config).unwrap();
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

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn global_config_test() {
//         use super::{config_init, Config};
//         config_init();

//         assert_eq!(Config::global().title, "config");
//         // println!("Config is {:#?}", Config::global());
//     }
// }
