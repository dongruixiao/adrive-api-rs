pub mod apis;
pub mod config;
pub mod data;
pub mod http;
pub mod utils;

use reqwest::Client;
use std::cell::RefCell;
use std::error::Error;
use std::sync::Arc
use std::result;

use crate::apis::credentials::Credentials;
use crate::apis::info::Info;
use crate::apis::signature::Signature;

pub type Result<T> = result::Result<T, Box<dyn Error>>;

pub struct ADriveAPI {
    client: Client,
    credentials: Arc<RefCell<Credentials>>,
    signature: Signature,
    info: Info,
}

impl ADriveAPI {
    pub fn new() -> Self {
        let client = reqwest::Client::new();
        let credentials = Arc::new(RefCell::new(Credentials::new()));
        let signature = Signature::new();
        Self {
            client,
            credentials,
            // signature: None,
            // info: None,
        }
    }
}
