pub mod configuration;
pub mod deployment;
pub mod kernel;
pub mod policy;
pub mod status;
pub mod telemetry;

use crate::dependency::State;

use dashmap::DashMap;
use once_cell::sync::Lazy;

pub static SERVICES: Lazy<DashMap<String, i32>> = Lazy::new(|| DashMap::new());

pub trait Service {
    fn new() -> ServiceStatus {
        ServiceStatus {
            componentName: "",
            version: "",
            fleetConfigArns: vec![],
            statusDetails: "",
            isRoot: false,
            state: State::FINISHED,
        }
    }
    fn enable() -> bool;
    // fn disable() -> bool;
    // fn start() -> bool;
    // fn restart() -> bool;
    // fn stop() -> bool;
    fn status() -> State {
        State::FINISHED
    }
}

pub struct ServiceStatus {
    componentName: &'static str,
    version: &'static str,
    fleetConfigArns: Vec<String>,
    statusDetails: &'static str,
    // We need to add this since during serialization, the 'is' is removed.
    isRoot: bool,
    state: State,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    use crate::services::{Service, SERVICES};

    use super::deployment::Deployments;
    use super::kernel::Kernel;
    use super::policy::Policy;
    use super::telemetry::Telemetry;
    use crate::dependency::State;

    #[test]
    fn services_test() {
        Kernel::new();
        assert_eq!(Kernel::status(), State::FINISHED);
    }
}
