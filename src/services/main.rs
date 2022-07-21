use crate::services::{Service, SERVICES};

const VERSION: &str = "";
const NAME: &str = "main";
pub struct Main {}

impl Service for Main {
    fn enable() {
        SERVICES.insert(NAME.to_string(), Self::new(NAME, VERSION));
    }
}
