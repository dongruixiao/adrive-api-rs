use std::{error, fmt};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
    #[serde(rename = "requestId")]
    pub request_id: Option<String>,
}

impl error::Error for ErrorResponse {}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "err type: {}, err message: {}", self.code, self.message)
    }
}
