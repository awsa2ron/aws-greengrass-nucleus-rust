#![allow(non_snake_case)]
//! The setup script is intended to give a brand new user of Greengrass to get started with Greengrass device quickly.
//! As part of that experience the user can get a fat bin for the Greengrass Nucleus, the script can launch the Nucleus
//! with the customer's provided config if desired, optionally provision the test device as an AWS IoT Thing, create and
//! attach policies and certificates to it, create TES role and role alias or uses existing ones and attaches
//! them to the IoT thing certificate.
use crate::{services, Args};

use anyhow::{Context, Error, Ok, Result};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_iot::{model::KeyPair, Client};
use aws_types::region::Region;
use rumqttc::{AsyncClient, ClientError, QoS};
use serde_json::json;
use std::path::Path;
use std::{fs, path::PathBuf};
use tracing::{debug, event, info, span, Level};



const IOT_ROLE_POLICY_NAME_PREFIX: &str = "GreengrassTESCertificatePolicy";
const GREENGRASS_CLI_COMPONENT_NAME: &str = "aws.greengrass.Cli";
const INITIAL_DEPLOYMENT_NAME_FORMAT: &str = "Deployment for %s";
const IAM_POLICY_ARN_FORMAT: &str = "arn:%s:iam::%s:policy/%s";
const MANAGED_IAM_POLICY_ARN_FORMAT: &str = "arn:%s:iam::aws:policy/%s";
