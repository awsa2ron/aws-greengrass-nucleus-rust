#![allow(unused)]
#![allow(non_snake_case)]
// #![doc = include_str!("../README.md")]

pub mod config;
pub mod dependency;
pub mod easysetup;
pub mod mqtt;
pub mod provisioning;
pub mod util;

pub mod services;

// pub use self::easysetup::perform_setup;
pub use self::mqtt::publish;
pub use self::services::status::upload_fss_data as fleet_status;

use clap::Parser;
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    // The AWS Region to use. The AWS IoT Greengrass Core software uses this Region
    // to retrieve or create the AWS resources that it requires
    #[clap(long, default_value = "ap-southeast-1")]
    pub aws_region: String,

    // (Optional) The path to the folder to use as the root for the AWS IoT Greengrass Core
    // software. Defaults to ~/.greengrass.
    #[clap(long, default_value = "/greengrass/v2")]
    pub root: String,

    // (Optional) The path to the configuration file that you use to run the AWS
    // IoT Greengrass Core software
    // software. Defaults to ~/.greengrass
    #[clap(long, default_value = "~/.greengrass")]
    pub init_config: String,

    // (Optional) Specify true or false. If true, the AWS IoT Greengrass Core software registers this
    // device as an AWS IoT thing, and provisions the AWS resources that the software requires. The
    // software provisions an AWS IoT thing, (optional) an AWS IoT thing group, a Thing Policy, an
    // IAM role, and an AWS IoT role alias. Defaults to false.
    #[clap(long)]
    pub provision: bool,

    // (Optional) The name of the AWS IoT thing that you register as this core device.
    // If the thing with
    // this name doesn't exist in your AWS account, the AWS IoT Greengrass Core software creates it.
    //Defaults to GreengrassV2IotThing_ plus a random UUID.
    #[clap(short, long)]
    pub thing_name: String,

    // (Optional) The name of the AWS IoT thing group where you add this core
    // device's AWS IoT thing.
    // If a deployment targets this thing group, this core device receives that deployment when it
    // connects to AWS IoT Greengrass. If the thing group with this name doesn't exist in your AWS
    // account, the AWS IoT Greengrass Core software creates it. Defaults to no thing group.
    #[clap(long)]
    pub thing_group_name: Option<String>,

    // (Optional) The name of the AWS IoT Policy to attach to the core device's
    // AWS IoT thing.
    // If specified, then the supplied thing_policy_name is attached to the provisioned IoT Thing.
    // Otherwise a policy called GreengrassV2IoTThingPolicy is used instead. If the policy with
    // this name doesn't exist in your AWS account, the AWS IoT Greengrass Core software creates it
    // with a default policy document.
    #[clap(long, default_value = "GreengrassV2IoTThingPolicy")]
    pub thing_policy_name: String,

    // (Optional) The name of the IAM role to use to acquire AWS credentials that let the device
    // interact with AWS services. If the role with this name doesn't exist in your AWS account, " the AWS
    // IoT Greengrass Core software creates it with the GreengrassV2TokenExchangeRoleAccess policy.
    // This role doesn't have access to your S3 buckets where you host component artifacts. So, you
    // must add permissions to your artifacts' S3 buckets and objects when you create a component.
    // Defaults to GreengrassV2TokenExchangeRole.
    #[clap(long, default_value = "GreengrassV2TokenExchangeRole")]
    pub tes_role_name: String,

    // (Optional) The name of the AWS IoT role alias that points to the IAM role that provides AWS
    // credentials for this device. If the role alias with this name doesn't exist in your AWS "
    // account, the
    // AWS IoT Greengrass Core software creates it and points it to the IAM role that you specify.
    // Defaults to GreengrassV2TokenExchangeRoleAlias.
    #[clap(long, default_value = "GreengrassV2TokenExchangeRoleAlias")]
    pub tes_role_alias_name: String,

    // (Optional) Specify true or false. If true, then the AWS IoT Greengrass Core software sets
    // itself up as a system service that runs when this device boots. The system service name is "
    // greengrass.
    // Defaults to false.
    #[clap(long)]
    pub setup_system_service: bool,

    // (Optional) The name of ID of the system user and group that the AWS IoT Greengrass Core
    // software uses to run components. This argument accepts the user and group separated by a
    // colon, where the group is optional. For example, you can specify ggc_user:ggc_group or
    // ggc_user.
    // * If you run as root, this defaults to the user and group that the config file defines. If the config
    // file doesn't define a user and group, this defaults to ggc_user:ggc_group. If ggc_user or
    // ggc_group don't exist, the software creates them.
    // * If you run as a non_root user, the AWS IoT Greengrass Core software uses that user to run "
    // components.
    // * If you don't specify a group, the AWS IoT Greengrass Core software uses the primary group
    // of the system user
    #[clap(long)]
    pub component_default_user: Option<String>,

    // (Optional) Specify true or false. If true, the AWS IoT Greengrass Core software retrieves and
    // deploys the Greengrass CLI component. Specify true to set up this core
    // device for local development. Specify false to set up this core device in a production
    // environment. Defaults to false.
    #[clap(long)]
    pub deploy_dev_tools: bool,

    // (Optional) Specify true or false. If true, the AWS IoT Greengrass Core software runs setup steps,
    // (optional) provisions resources, and starts the software. If false, the software runs only setup
    // steps and (optional) provisions resources. Defaults to true.
    #[clap(long)]
    pub start: bool,

    // (Optional) Path of a plugin jar file. The plugin will be included as "
    // trusted plugin in nucleus. Specify multiple times for including multiple plugins.;
    #[clap(long)]
    pub trusted_plugin: Option<String>,
}
