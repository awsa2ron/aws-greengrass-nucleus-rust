use crate::dependency::State;
use crate::services::{Service, ServiceStatus, SERVICES};

const VERSION: &str = "0.0.0";
pub struct Deployments {}

impl Service for Deployments {
    fn enable() -> bool {
        SERVICES.insert("DeploymentService".to_string(), 3);
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
