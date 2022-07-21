use crate::services::{Service, SERVICES};

const VERSION: &str = "0.0.0";
pub struct Telemetry {}

impl Service for Telemetry {
    fn enable() -> bool {
        SERVICES.insert("TelemetryAgent".to_string(), 4);
        true
    }
}
