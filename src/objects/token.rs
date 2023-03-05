use crate::utils::deser::rfc3339_string_as_datetime;

use chrono::{DateTime, Utc};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Debug)]
pub struct RefreshTokenRequest<'a> {
    pub refresh_token: &'a str,
}

#[derive(Deserialize, Derivative)]
#[derivative(Debug)]
pub struct RefreshTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,

    #[serde(deserialize_with = "rfc3339_string_as_datetime")]
    pub expire_time: DateTime<Utc>,
    pub expires_in: i64,

    pub default_sbox_drive_id: String,
    pub default_drive_id: String,

    #[serde(flatten)]
    #[derivative(Debug = "ignore")]
    pub extra: HashMap<String, serde_json::Value>,
}
