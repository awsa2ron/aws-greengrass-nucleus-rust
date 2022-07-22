use crate::services::{Service, SERVICES};

const VERSION: &str = "0.0.0";
const NAME: &str = "UpdateSystemPolicyService";
pub struct Policy {}

impl Service for Policy {
    fn enable() {
        SERVICES.insert(NAME.to_string(), Self::new(NAME, VERSION));
    }
}
