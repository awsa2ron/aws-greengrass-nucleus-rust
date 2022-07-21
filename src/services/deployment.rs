use crate::services::{Service, SERVICES};

const VERSION: &str = "0.0.0";
pub struct Deployments {}

impl Service for Deployments {
    fn enable() -> bool {
        SERVICES.insert("DeploymentService".to_string(), 3);
        true
    }
}
