use anyhow::{Error, Ok, Result};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_iot::{model::KeyPair, Client};
use aws_types::region::Region;

pub async fn init(region: &str) -> Result<Client, Error> {
    let region_provider = RegionProviderChain::first_try(Region::new(region.to_string()))
        .or_default_provider()
        .or_else(Region::new("ap-southeast-1"));
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    Ok(Client::new(&shared_config))
}
