use crate::services::{Service, SERVICES};

pub const VERSION: &str = "2.5.6";
const NAME: &str = "aws.greengrass.Nucleus";
pub struct Kernel {}

impl Service for Kernel {
    fn enable() {
        SERVICES.insert(NAME.to_string(), Self::new(NAME, VERSION));
    }
}

pub fn new() {}
