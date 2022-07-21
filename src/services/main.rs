use crate::services::{Service, ServiceStatus, SERVICES};

const VERSION: &str = "2.5.5";
const NAME: &str = "main";
pub struct Main {}

impl Service for Main {
    fn enable() {
        SERVICES.insert(NAME.to_string(), Main::new(NAME));
    }
}
