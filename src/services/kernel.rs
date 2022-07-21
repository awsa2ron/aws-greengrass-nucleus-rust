use crate::services::{Service, SERVICES};

const VERSION: &str = "2.5.5";
const NAME: &str = "aws.greengrass.Nucleus";
pub struct Kernel {}

impl Service for Kernel {
    fn enable() {
        SERVICES.insert(NAME.to_string(), Self::new(NAME, VERSION));
    }
}
