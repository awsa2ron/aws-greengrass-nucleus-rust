#![allow(non_snake_case)]
//! # Fleet Status Service
//!
//! It is responsible for uploading the statuses of components within the device for all fleets.
//! The fleet status service will be gathering state from the device and providing it to the customers. The fleet status service is
//! an important feature to provide insight to the customer on the devices which have greengrass V2 running on them and
//! answering questions like:
//!   - Is the device running the greengrass application as expected?
//!   - What happened on the device after the recent deployment, did a service start failing/flapping after it?
//!
//! Based on these statuses, the customers can build their infrastructure to take appropriate actions.
//! The QoS for FSS is 1 since we need the information to reach the cloud at least once.
//!
//! There is an initial jitter added for each device to emit the periodic updates. This is to ensure not all the devices within the fleet
//! update their fleet statuses at the same time.
//!
//! Since there is a limit of 128 KB per message on IoT Core, the fleet status service will chunk the message to make sure
//! that each message does not exceed the max size of 128 KB.
//!
//! # Startup
//! 1. FleetStatusService starts as a greengrass service, and is by default enabled. It
//! starts a timer to update the information about all the components running
//! in the Nucleus after a specific interval.
//!
//! # Shutdown
//! Service lifecycle is managed by Nucleus. As part of Nucleus shutdown, FSS cancels the timer for cadence based data
//!  upload.
//!
//! # Workflow
//! There are two conditions the Fleet Status Service will upload the status information to the cloud.
//! 1. Event-Triggered
//! - After deployments.
//! - If a component goes into BROKEN state in between deployments.
//! 2. Periodic/Cadence Based
//! - Default interval is 1 day.
//!
//! # Sample Configuration
//! ` Note: this configuration cannot be updated via deployments.`
//! ```text
//! services:
//!   main:
//!     dependencies:
//!       FleetStatusService
//!   FleetStatusService:
//!     configuration:
//!       periodicUpdateIntervalSec: 86400
//! ```

use crate::{config, dependency};
use anyhow::{Context, Error, Ok, Result};
use bytes::Bytes;
use clap::Args;
use rumqttc::{Publish, QoS};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::{
    sync::mpsc,
    time::{sleep, Duration},
};
use tracing::{debug, event, info, span, Level};

use crate::services::{Service, SERVICES};

use super::kernel;

const VERSION: &str = "";
const NAME: &str = "FleetStatusService";
pub struct Status {}

impl Service for Status {
    fn enable() {
        SERVICES.insert(NAME.to_string(), Self::new(NAME, VERSION));
    }
}

#[doc(alias = "uploadFleetStatusServiceData")]
pub async fn upload_fss_data(tx: mpsc::Sender<Publish>) -> Result<()> {
    let name = config::Config::global().system.thingName.as_str();

    tokio::spawn(async move {
        loop {
            // println!("status service.");
            let topic = format!("$aws/things/{}/greengrassv2/health/json", name);
            let payload = json!(fss_data(name)).to_string();
            info!(
                event = "fss-status-update-published",
                "Status update published to FSS"
            );
            tx.send(Publish {
                dup: false,
                qos: QoS::AtMostOnce,
                retain: false,
                pkid: 0,
                topic: topic.to_string(),
                payload: Bytes::from(payload.to_string()),
            })
            .await;
            sleep(Duration::from_secs(86400)).await;
        }
    });
    Ok(())
}

// implements Chunkable<ComponentStatusDetails>
#[derive(Serialize, Deserialize, Debug)]
pub struct FleetStatusDetails {
    ggcVersion: &'static str,
    platform: &'static str,
    architecture: &'static str,
    thing: String,
    overallDeviceStatus: OverallStatus,
    sequence_number: usize,
    pub components: Vec<crate::services::ServiceStatus>,
    // components: Vec<ComponentStatusDetails>,
    // deploymentInformation: String,
    // pub void setVariablePayload(List<ComponentStatusDetails> variablePayload) {
    //     this.setComponentStatusDetails(variablePayload),
    // }
}

impl FleetStatusDetails {
    pub fn new(name: &str) -> Self {
        FleetStatusDetails {
            ggcVersion: kernel::VERSION,
            platform: "linux",
            architecture: "x86_64",
            thing: name.to_string(),
            overallDeviceStatus: OverallStatus::HEALTHY,
            sequence_number: 9,
            // deploymentInformation: "".to_string(),
            components: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum OverallStatus {
    HEALTHY,
    UNHEALTHY,
}

// pub const detailedStatus: &str = "detailedStatus";
// pub const failureCause: &str = "failureCause";

#[derive(Serialize, Deserialize, Debug)]
struct StatusDetails {
    pub DETAILED_STATUS_KEY: String,
    pub FAILURE_CAUSE_KEY: String,
}

// pub const STATUS_KEY: &str = "status";
// pub const STATUS_DETAILS_KEY: &str = "status_details";
// pub const ARN_FOR_STATUS_KEY: &str = "fleetConfigurationArnForStatus";
#[derive(Serialize, Deserialize, Debug)]
pub struct DeploymentInformation {
    status: String,
    status_details: StatusDetails,
    fleetConfigurationArnForStatus: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ComponentStatusDetails {
    component_name: String,

    version: String,

    fleetconfig_arns: Vec<String>,

    status_details: String,

    // We need to add this since during serialization, the 'is' is removed.
    is_root: bool,

    status: dependency::State,
}

pub const FLEET_STATUS_SERVICE_TOPICS: &str = "FleetStatusService";
pub const DEFAULT_FLEET_STATUS_SERVICE_PUBLISH_TOPIC: &str =
    "$aws/things/{thing_name}/greengrassv2/health/json";
pub const FLEET_STATUS_TEST_PERIODIC_UPDATE_INTERVAL_SEC: &str = "fssPeriodicUpdateIntervalSec";
pub const DEFAULT_PERIODIC_PUBLISH_INTERVAL_SEC: usize = 86_400;
pub const FLEET_STATUS_PERIODIC_PUBLISH_INTERVAL_SEC: &str = "periodicStatusPublishIntervalSeconds";
const FLEET_STATUS_SEQUENCE_NUMBER_TOPIC: &str = "sequenceNumber";
const FLEET_STATUS_LAST_PERIODIC_UPDATE_TIME_TOPIC: &str = "lastPeriodicUpdateTime";
const MAX_PAYLOAD_LENGTH_BYTES: usize = 128_000;
pub const DEVICE_OFFLINE_MESSAGE: &str =
    "Device not configured to talk to AWS IoT cloud. FleetStatusService is offline";
// extends GreengrassService
pub struct FleetStatusService {
    // DeviceConfiguration deviceConfiguration;
    // GlobalStateChangeListener handleServiceStateChange = this::handleServiceStateChange,
    // Function<Map<String, Object>, Boolean> deploymentStatusChanged = this::deploymentStatusChanged,
    update_topic: String,
    thing_name: String,
    // MqttClient mqttClient,
    // Kernel kernel,
    architecture: String,
    platform: String,
    // MqttChunkedPayloadPublisher<ComponentStatusDetails> publisher,
    // DeploymentStatusKeeper deploymentStatusKeeper,
    //For testing

    // AtomicBoolean isConnected = new AtomicBoolean(true),
    // AtomicBoolean isEventTriggeredUpdateInProgress = new AtomicBoolean(false),
    // AtomicBoolean isFSSSetupComplete = new AtomicBoolean(false),
    // Set<GreengrassService> updatedGreengrassServiceSet =
    // Collections.newSetFromMap(new ConcurrentHashMap<>()),
    // ConcurrentHashMap<GreengrassService, Instant> serviceFssTracksMap = new ConcurrentHashMap<>(),
    // AtomicBoolean isDeploymentInProgress = new AtomicBoolean(false),
    // Object periodicUpdateInProgressLock = new Object(),
    // periodicPublishIntervalSec: usize,
    // ScheduledFuture<?> periodicUpdateFuture,
}

pub fn fss_data(
    name: &str, // overAllStatus: OverallStatus,
                // deploymentInformation: DeploymentInformation,
) -> FleetStatusDetails {
    // if (!isConnected.get()) {
    // if true {
    //     info!("Not updating fleet status data since MQTT connection is interrupted.");
    //     return;
    // }
    let mut components: ComponentStatusDetails;
    let sequence_number: usize;

    // synchronized (greengrassServiceSet)

    let mut payload = FleetStatusDetails::new(name);
    SERVICES
        .iter()
        .for_each(|r| payload.components.push(r.value().clone()));
    // let fleetStatusDetails = FleetStatusDetails::new();
    // let serde_string = serde_json::to_string(&fleetStatusDetails).unwrap();
    // info!(
    //     event = "fss-status-update-published",
    //     "fleetStatusDetails {:?}", serde_string
    // );
    // info!("fss-status-update-published").log("fleetStatusDetails {} components {}");
    // fleetStatusDetails, components);

    // publisher.publish(fleetStatusDetails, components);

    // println!("{}", json!(payload));
    payload

    // serde_string
}

#[doc(alias = "deploymentStatusChanged")]
pub fn deployment_status_changed(
    name: &str, // overAllStatus: OverallStatus,
                // deploymentInformation: DeploymentInformation,
) -> bool {
    true
}
