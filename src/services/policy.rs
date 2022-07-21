use crate::services::{Service, ServiceStatus, SERVICES};

const VERSION: &str = "0.0.0";
const NAME: &str = "UpdateSystemPolicyService";
pub struct Policy {}

impl Service for Policy {
    fn enable() {
        SERVICES.insert("UpdateSystemPolicyService".to_string(), Policy::new(NAME));
    }
}
