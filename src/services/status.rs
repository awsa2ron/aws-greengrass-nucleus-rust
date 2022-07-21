//! It is responsible for uploading the statuses of components within the device for all fleets.
//! The fleet status service will be gathering state from the device and providing it to the customers. The fleet status service is
//! an important feature to provide insight to the customer on the devices which have greengrass V2 running on them and
//! answering questions like:
//!   > Is the device running the greengrass application as expected?
//!   > What happened on the device after the recent deployment, did a service start failing/flapping after it?
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
//! 1. [***FleetStatusService***](FleetStatusService.java) starts as a greengrass service, and is by default enabled. It
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
// # Sample Configuration
// > Note: this configuration cannot be updated via deployments.
// ```
// services:
//   main:
//     dependencies:
//       FleetStatusService
//   FleetStatusService:
//     configuration:
//       periodicUpdateIntervalSec: 86400
// ```

use crate::dependency;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{debug, event, info, span, Level};

use crate::services::{Service, ServiceStatus, SERVICES};

const VERSION: &str = "";
const NAME: &str = "FleetStatusService";
pub struct Status {}

impl Service for Status {
    fn enable() {
        SERVICES.insert("FleetStatusService".to_string(), Status::new(NAME, VERSION));
    }

    fn start() {
        println!("Status start...");
        let mut payload = FleetStatusDetails::new();
        // payload.components.push(SERVICES.get("FleetStatusService").unwrap().value().clone());
        SERVICES
            .iter()
            .for_each(|r| payload.components.push(r.value().clone()));

        println!("{}", json!(payload));
    }
}

// implements Chunkable<ComponentStatusDetails>
#[derive(Serialize, Deserialize, Debug)]
pub struct FleetStatusDetails {
    ggcVersion: &'static str,

    platform: &'static str,

    architecture: &'static str,

    thing: String,
    overallStatus: OverallStatus,

    sequenceNumber: usize,

    components: Vec<crate::services::ServiceStatus>,
    // components: Vec<ComponentStatusDetails>,
    deploymentInformation: String,
    // pub void setVariablePayload(List<ComponentStatusDetails> variablePayload) {
    //     this.setComponentStatusDetails(variablePayload),
    // }
}

impl FleetStatusDetails {
    pub fn new() -> Self {
        FleetStatusDetails {
            ggcVersion: "2.5.5",
            platform: "linux",
            architecture: "x86_64",
            thing: "".to_string(),
            overallStatus: OverallStatus::HEALTHY,
            sequenceNumber: 5,
            deploymentInformation: "".to_string(),
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
// pub const STATUS_DETAILS_KEY: &str = "statusDetails";
// pub const ARN_FOR_STATUS_KEY: &str = "fleetConfigurationArnForStatus";
#[derive(Serialize, Deserialize, Debug)]
pub struct DeploymentInformation {
    status: String,
    statusDetails: StatusDetails,
    fleetConfigurationArnForStatus: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ComponentStatusDetails {
    componentName: String,

    version: String,

    fleetConfigArns: Vec<String>,

    statusDetails: String,

    // We need to add this since during serialization, the 'is' is removed.
    isRoot: bool,

    state: dependency::State,
}

pub const FLEET_STATUS_SERVICE_TOPICS: &str = "FleetStatusService";
pub const DEFAULT_FLEET_STATUS_SERVICE_PUBLISH_TOPIC: &str =
    "$aws/things/{thingName}/greengrassv2/health/json";
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
    updateTopic: String,
    thingName: String,
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
    periodicPublishIntervalSec: usize,
    // ScheduledFuture<?> periodicUpdateFuture,
}

pub fn uploadFleetStatusServiceData(// overAllStatus: OverallStatus,
    // deploymentInformation: DeploymentInformation,
) -> String {
    // if (!isConnected.get()) {
    // if true {
    //     info!("Not updating fleet status data since MQTT connection is interrupted.");
    //     return;
    // }
    let mut components: ComponentStatusDetails;
    let sequenceNumber: usize;

    // synchronized (greengrassServiceSet)

    let fleetStatusDetails = FleetStatusDetails::new();
    let serde_string = serde_json::to_string(&fleetStatusDetails).unwrap();
    info!(
        event = "fss-status-update-published",
        "fleetStatusDetails {:?}", serde_string
    );
    // info!("fss-status-update-published").log("fleetStatusDetails {} components {}");
    // fleetStatusDetails, components);

    // util::publish(client, "hello/world") // easysetup::createThing(client, &name, &name),
    // publisher.publish(fleetStatusDetails, components);
    info!(
        event = "fss-status-update-published",
        "Status update published to FSS"
    );

    serde_string
}
