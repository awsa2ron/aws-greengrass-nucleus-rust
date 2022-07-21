use crate::services::{Service, ServiceStatus, SERVICES};

const VERSION: &str = "0.0.0";
pub struct Telemetry {}

impl Service for Telemetry {
    fn enable() {
        SERVICES.insert("TelemetryAgent".to_string(), Telemetry::new());
    }
}
