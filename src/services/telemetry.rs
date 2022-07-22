use crate::services::{Service, SERVICES};

const VERSION: &str = "0.0.0";
const NAME: &str = "TelemetryAgent";
pub struct Telemetry {}

impl Service for Telemetry {
    fn enable() {
        SERVICES.insert(NAME.to_string(), Self::new(NAME, VERSION));
    }
}
