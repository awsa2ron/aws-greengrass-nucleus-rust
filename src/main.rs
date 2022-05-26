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



fn main() {
    println!("Hello, world!");
}
