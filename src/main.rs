use anyhow::Result;
use aws_config::meta::region::RegionProviderChain;
use aws_greengrass_nucleus::greengrassv2;
use aws_sdk_greengrassv2::{Client, Error, Region, PKG_VERSION};
use clap::Parser;
use serde::{Deserialize, Serialize};
use tracing::{debug, event, info, span, Level};
use tracing_subscriber;

// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // thing-name
    #[clap(short, long)]
    name: String,

    #[clap(long, default_value = "/greengrass/v2")]
    root: String,

    #[clap(short, long, default_value = "FILE")]
    log: String,

    #[clap(short, long)]
    region: Option<String>,

    #[clap(short, long)]
    provision: bool,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // GreengrassSetup greengrassSetup = new GreengrassSetup(System.out, System.err, args);
    // // try {
    //     greengrassSetup.parseArgs();
    //     greengrassSetup.performSetup();
    // // } catch (Throwable t) {
    // //     logger.atError().setCause(t).log("Error while trying to setup Greengrass Nucleus");
    // //     System.err.println("Error while trying to setup Greengrass Nucleus");
    // //     t.printStackTrace(greengrassSetup.errStream);
    // //     System.exit(1);
    // // }

    let args = Args::parse();
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();

    // downloadRootCAToFile("rootCA.pem");

    // iot_list_polices(&client).await
    let region_provider = RegionProviderChain::first_try(args.region.map(Region::new))
        .or_default_provider()
        .or_else(Region::new("us-west-2"));

    let shared_config = aws_config::from_env().region(region_provider).load().await;
    greengrassv2::ggv2_init(&shared_config).await
}
