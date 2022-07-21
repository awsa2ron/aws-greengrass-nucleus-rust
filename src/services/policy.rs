use crate::dependency::State;
use crate::services::{Service, ServiceStatus, SERVICES};

const VERSION: &str = "0.0.0";
pub struct Policy {}

impl Service for Policy {
    fn enable() -> bool {
        SERVICES.insert("UpdateSystemPolicyService".to_string(), 2);
        true
    }
    fn status() -> ServiceStatus {
        ServiceStatus {
            componentName: "",
            version: "",
            fleetConfigArns: vec![],
            statusDetails: "",
            isRoot: false,
            state: State::FINISHED,
        }
    }
}
