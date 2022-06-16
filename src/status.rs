use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{debug, event, info, span, Level};
// implements Chunkable<ComponentStatusDetails>
#[derive(Serialize, Deserialize, Debug)]
pub struct FleetStatusDetails {
    ggcVersion: &'static str,

    platform: &'static str,

    architecture: &'static str,

    thing: String,
    overallStatus: OverallStatus,

    sequenceNumber: usize,

    // List<ComponentStatusDetails> componentStatusDetails,
    deploymentInformation: String,
    // deploymentInformation: DeploymentInformation,
    // pub void setVariablePayload(List<ComponentStatusDetails> variablePayload) {
    //     this.setComponentStatusDetails(variablePayload),
    // }
}

impl FleetStatusDetails {
    fn new() -> Self {
        FleetStatusDetails {
            ggcVersion: "2.5.5",
            platform: "linux",
            architecture: "x86_64",
            thing: "".to_string(),
            overallStatus: OverallStatus::HEALTHY,
            sequenceNumber: 5,
            deploymentInformation: "".to_string(),
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

pub struct ComponentStatusDetails {
    componentName: String,

    version: String,

    // List<String> fleetConfigArns,
    statusDetails: String,

    // We need to add this since during serialization, the 'is' is removed.
    isRoot: bool,
    // State state,
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
    // List<ComponentStatusDetails> components = new ArrayList<>();
    // long sequenceNumber;

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
