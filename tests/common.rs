use adrive_api_rs::{ADriveAPI, ADriveCoreAPI};
use std::sync::OnceLock;

pub static ADRIVE_API: OnceLock<ADriveAPI> = OnceLock::new();
pub static ADRIVE_CORE_API: OnceLock<ADriveCoreAPI> = OnceLock::new();

pub fn adrive_api() -> &'static ADriveAPI {
    ADRIVE_API.get_or_init(ADriveAPI::new)
}

pub fn adrive_core_api() -> &'static ADriveCoreAPI {
    ADRIVE_CORE_API.get_or_init(ADriveCoreAPI::new)
}
