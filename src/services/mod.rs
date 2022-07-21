pub mod configuration;
pub mod deployment;
pub mod kernel;
pub mod policy;
pub mod status;
pub mod telemetry;

use dashmap::DashMap;
use once_cell::sync::Lazy;

pub static SERVICES: Lazy<DashMap<String, i32>> = Lazy::new(|| DashMap::new());

pub trait Service {
    fn enable() -> bool;
    // fn disable() -> bool;
    // fn start() -> bool;
    // fn restart() -> bool;
    // fn stop() -> bool;
    // fn status() -> bool;
}

use deployment::Deployments;
use kernel::Kernel;
use policy::Policy;
use telemetry::Telemetry;

fn services_init() {
    Kernel::enable();
    Telemetry::enable();
    Deployments::enable();
    Policy::enable();
    println!("{:?}", *SERVICES.get("Kernel").unwrap());
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

    #[test]
    fn services_init_test() {
        Kernel::enable();
        Telemetry::enable();
        Deployments::enable();
        Policy::enable();
        assert_eq!(*SERVICES.get("aws.greengrass.Nucleus").unwrap(), 0);
        assert_eq!(*SERVICES.get("UpdateSystemPolicyService").unwrap(), 2);
        assert_eq!(*SERVICES.get("DeploymentService").unwrap(), 3);
        assert_eq!(*SERVICES.get("TelemetryAgent").unwrap(), 4);
    }
}
