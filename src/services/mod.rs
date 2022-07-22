pub mod deployment;
pub mod kernel;
pub mod main;
pub mod policy;
pub mod status;
pub mod telemetry;

use crate::dependency::State;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use dashmap::DashMap;
use once_cell::sync::Lazy;

pub static SERVICES: Lazy<DashMap<String, ServiceStatus>> = Lazy::new(|| DashMap::new());

/// ```
/// /// Some documentation.
/// # fn foo() {} // this function will be hidden
/// println!("Hello, World!");
/// let foo = "foo";
/// assert_eq!(foo, "foo");
/// ```

pub trait Service {
    fn new(name: &'static str, ver: &'static str) -> ServiceStatus {
        ServiceStatus {
            componentName: name,
            version: ver,
            fleetConfigArns: vec![],
            statusDetails: json!(null),
            isRoot: false,
            status: State::FINISHED,
        }
    }
    fn enable();
    // fn disable() -> bool;
    // fn start() {}
    // fn restart() -> bool;
    // fn stop() -> bool;
    // fn status() -> State {}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServiceStatus {
    componentName: &'static str,
    version: &'static str,
    fleetConfigArns: Vec<String>,
    statusDetails: Value,
    // We need to add this since during serialization, the 'is' is removed.
    isRoot: bool,
    status: State,
}

use deployment::Deployments;
use kernel::Kernel;
use main::Main;
use policy::Policy;
use status::Status;
use telemetry::Telemetry;

pub fn start_services() {
    Kernel::enable();
    Main::enable();
    Policy::enable();
    Deployments::enable();
    Telemetry::enable();
    Status::enable();
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
