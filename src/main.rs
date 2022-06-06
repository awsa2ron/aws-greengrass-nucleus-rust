use clap::Parser;

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

pub const COMMAND:&str = r#"$ sudo -E java -Droot="/greengrass/v2" -Dlog.store=FILE -jar ./GreengrassCore/lib/Greengrass.jar 
                            --aws-region ap-southeast-1 --thing-name GreengrassQuickStartCore-1 --component-default-user ggc_user:ggc_group 
                            --provision true
                        "#;
pub const FLOW:&str = r#"Provisioning AWS IoT resources for the device with IoT Thing Name: [GreengrassQuickStartCore-new]...
                        Found IoT policy "GreengrassV2IoTThingPolicy", reusing it
                        Creating keys and certificate...
                        Attaching policy to certificate...
                        Creating IoT Thing "GreengrassQuickStartCore-new"...
                        Attaching certificate to IoT thing...
                        Successfully provisioned AWS IoT resources for the device with IoT Thing Name: [GreengrassQuickStartCore-new]!
                        Setting up resources for aws.greengrass.TokenExchangeService ... 
                        Attaching TES role policy to IoT thing...
                        No managed IAM policy found, looking for user defined policy...
                        IAM policy named "GreengrassV2TokenExchangeRoleAccess" already exists. Please attach it to the IAM role if not already
                        Configuring Nucleus with provisioned resource details...
                        Root CA file found at "/greengrass/v2/rootCA.pem". Contents will be preserved.
                        Downloading Root CA from "https://www.amazontrust.com/repository/AmazonRootCA1.pem"
                        Created device configuration
                        Successfully configured Nucleus with provisioned resource details!
                        Launching Nucleus...
                        Launched Nucleus successfully.
                        "#;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // thing-name
    #[clap(short, long)]
    name: String,

    #[clap(short, long, default_value = "/greengrass/v2")]
    root: String,

    #[clap(short, long, default_value = "FILE")]
    log: String,

    #[clap(short, long, default_value = "ap-southeast-1")]
    aws_region: String,

    #[clap(short, long)]
    provision: bool,
}

fn main() {
    let args = Args::parse();

    println!("{} {} {} {} {}", args.name, args.root, args.log, args.aws_region, args.provision)
}
