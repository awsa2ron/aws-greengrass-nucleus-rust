use aws_config::meta::region::RegionProviderChain;
use aws_sdk_greengrassv2::{Client, Error, Region, PKG_VERSION};
use tracing::{debug, event, info, span, Level};

pub async fn ggv2_init(shared_config: &aws_types::SdkConfig) -> Result<(), Error> {
    let client = Client::new(&shared_config);

    ggv2_list_core_devices(&client).await
}

// Lists your Greengrass V2 cores.
async fn ggv2_list_core_devices(client: &Client) -> Result<(), Error> {
    let resp = client.list_core_devices().send().await?;

    info!("ggv2_list_core_devices:");

    for core in resp.core_devices().unwrap() {
        info!(
            "  Name:  {}",
            core.core_device_thing_name().unwrap_or_default()
        );
        info!("  Status:  {:?}", core.status().unwrap());
        info!(
            "  Last update:  {:?}",
            core.last_status_update_timestamp().unwrap()
        );
    }

    Ok(())
}
