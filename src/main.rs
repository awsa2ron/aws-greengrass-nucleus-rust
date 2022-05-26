pub enum OverallStatus {
    HEALTHY,
    UNHEALTHY
}

pub const detailedStatus: &str = "detailedStatus";
pub const failureCause: &str = "failureCause";


struct StatusDetails {
    pub DETAILED_STATUS_KEY:String,
    pub FAILURE_CAUSE_KEY:String,
}
// implements Chunkable<ComponentStatusDetails> 
pub struct  FleetStatusDetails{
    ggcVersion:String,

    platform:String,

    architecture:String,

    thing:String,

    overallStatus:OverallStatus ,

    sequenceNumber:usize,

    // List<ComponentStatusDetails> componentStatusDetails,

    // DeploymentInformation deploymentInformation,

    // pub void setVariablePayload(List<ComponentStatusDetails> variablePayload) {
    //     this.setComponentStatusDetails(variablePayload),
    // }
}

pub const STATUS_KEY:&str = "status";
pub const STATUS_DETAILS_KEY:&str = "statusDetails";
pub const ARN_FOR_STATUS_KEY:&str = "fleetConfigurationArnForStatus";
pub struct  DeploymentInformation {
    status:String,
    statusDetails:StatusDetails,
    fleetConfigurationArnForStatus:String,
}

pub struct  ComponentStatusDetails {
    componentName:String,

    version:String,

    // List<String> fleetConfigArns,

    statusDetails:String,

    // We need to add this since during serialization, the 'is' is removed.
    isRoot:bool,

    // State state,
}

pub const FLEET_STATUS_SERVICE_TOPICS:&str = "FleetStatusService";
pub const DEFAULT_FLEET_STATUS_SERVICE_PUBLISH_TOPIC:&str = "$aws/things/{thingName}/greengrassv2/health/json";
pub const FLEET_STATUS_TEST_PERIODIC_UPDATE_INTERVAL_SEC :&str= "fssPeriodicUpdateIntervalSec";
pub const DEFAULT_PERIODIC_PUBLISH_INTERVAL_SEC:usize = 86_400;
pub const FLEET_STATUS_PERIODIC_PUBLISH_INTERVAL_SEC:&str = "periodicStatusPublishIntervalSeconds";
const  FLEET_STATUS_SEQUENCE_NUMBER_TOPIC:&str = "sequenceNumber";
const  FLEET_STATUS_LAST_PERIODIC_UPDATE_TIME_TOPIC:&str = "lastPeriodicUpdateTime";
const  MAX_PAYLOAD_LENGTH_BYTES:usize = 128_000;
pub const  DEVICE_OFFLINE_MESSAGE:&str = "Device not configured to talk to AWS IoT cloud. FleetStatusService is offline";
// extends GreengrassService 
pub struct  FleetStatusService{
    // DeviceConfiguration deviceConfiguration;
    // GlobalStateChangeListener handleServiceStateChange = this::handleServiceStateChange,
    // Function<Map<String, Object>, Boolean> deploymentStatusChanged = this::deploymentStatusChanged,

    updateTopic:String,
    thingName:String,
    // MqttClient mqttClient,
    // Kernel kernel,
    architecture:String,
    platform:String,
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

    periodicPublishIntervalSec:usize,
    // ScheduledFuture<?> periodicUpdateFuture,

}



fn main() {
    println!("Hello, world!");
}
