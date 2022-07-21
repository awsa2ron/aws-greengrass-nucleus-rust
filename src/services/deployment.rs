use crate::services::{Service, ServiceStatus, SERVICES};

const VERSION: &str = "0.0.0";
const NAME: &str = "DeploymentService";
pub struct Deployments {}

impl Service for Deployments {
    fn enable() {
        SERVICES.insert("DeploymentService".to_string(), Deployments::new(NAME));
    }
}
