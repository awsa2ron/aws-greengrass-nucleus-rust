use crate::dependency::State;
use crate::services::{Service, ServiceStatus, SERVICES};

const VERSION: &str = "0.0.0";
pub struct Telemetry {}

impl Service for Telemetry {
    fn enable() -> bool {
        SERVICES.insert("TelemetryAgent".to_string(), 4);
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
