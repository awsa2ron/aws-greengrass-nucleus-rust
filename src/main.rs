use anyhow::{Result, Error};
use aws_greengrass_nucleus::{easysetup, provisioning};
use aws_config::meta::region::RegionProviderChain;
use aws_types::region::Region;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::path::Path;
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
    let args = Args::parse();

    tracing_subscriber::fmt::init();

    let region_provider = RegionProviderChain::first_try(args.region.map(Region::new))
        .or_default_provider()
        .or_else(Region::new("us-west-2"));

    provisioning::print_flow();
    provisioning::init(region_provider);

    easysetup::downloadRootCAToFile(Path::new("rootCA.pem")).await;

    loop {};
    Ok(())
}
