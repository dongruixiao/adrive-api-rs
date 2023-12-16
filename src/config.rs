use crate::data::credentials::Credentials;
use crate::data::signature::Signature;

pub struct Config {
    credentials: Option<Credentials>,
    signature: Option<Signature>,
    drive_id: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            credentials: None,
            signature: None,
            drive_id: None,
        }
    }
}
