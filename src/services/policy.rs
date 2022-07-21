use crate::services::{Service, SERVICES};

const VERSION: &str = "0.0.0";
pub struct Policy {}

impl Service for Policy {
    fn enable() -> bool {
        SERVICES.insert("UpdateSystemPolicyService".to_string(), 2);
        true
    }
}
