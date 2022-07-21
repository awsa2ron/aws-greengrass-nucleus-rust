pub mod configuration;
pub mod deployment;
pub mod kernel;
pub mod policy;
pub mod status;
pub mod telemetry;

use crate::dependency::State;
use serde::{Deserialize, Serialize};
use serde_json::json;

use dashmap::DashMap;
use once_cell::sync::Lazy;

pub static SERVICES: Lazy<DashMap<String, ServiceStatus>> = Lazy::new(|| DashMap::new());

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
    fn enable();
    // fn disable() -> bool;
    fn start() {}
    // fn restart() -> bool;
    // fn stop() -> bool;
    fn status() -> State {
        State::FINISHED
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceStatus {
    componentName: &'static str,
    version: &'static str,
    fleetConfigArns: Vec<String>,
    statusDetails: &'static str,
    // We need to add this since during serialization, the 'is' is removed.
    isRoot: bool,
    state: State,
}

use deployment::Deployments;
use kernel::Kernel;
use policy::Policy;
use status::Status;
use telemetry::Telemetry;

pub fn start_services() {
    Kernel::enable();
    Policy::enable();
    Deployments::enable();
    Telemetry::enable();
    Status::enable();
    // for (name, state) in SERVICES.iter() {
    //     println!("name is {name} and state is {state}");
    // }
    // let mut payload = status::FleetStatusDetails::new();
    // SERVICES.iter().for_each(|r|  payload.componentStatusDetails.push(json!(r.value())));
    // print!("Status is {}", json!(status::FleetStatusDetails::new()));
    SERVICES
        .iter()
        .for_each(|r| print!("key: {}, value: {}", r.key(), json!(r.value())));
    Status::start();
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
        Kernel::enable();
        assert_eq!(
            SERVICES
                .get("aws.greengrass.Nucleus")
                .unwrap()
                .componentName,
            ""
        );
        assert_eq!(SERVICES.get("aws.greengrass.Nucleus").unwrap().version, "");
        // assert_eq!(SERVICES.get("aws.greengrass.Nucleus").unwrap().fleetConfigArns, vec![]);
        // assert_eq!(SERVICES.get("aws.greengrass.Nucleus").unwrap().statusDetails, null);
        assert_eq!(
            SERVICES.get("aws.greengrass.Nucleus").unwrap().isRoot,
            false
        );
        assert_eq!(
            SERVICES.get("aws.greengrass.Nucleus").unwrap().state,
            State::FINISHED
        );
    }
}
