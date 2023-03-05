// Request URL: https://api.aliyundrive.com/adrive/v2/user/get

use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize)]
pub struct UserInfoPayload {}

#[derive(Derivative, Deserialize)]
#[derivative(Debug)]
pub struct UserInfo {
    pub user_id: String,
    pub avatar: String,
    pub email: String,
    pub nick_name: String,
    pub phone: String,
    pub description: String,
    pub default_drive_id: String,

    #[serde(deserialize_with = "ts_milliseconds::deserialize")]
    pub created_at: DateTime<Utc>,
    #[serde(deserialize_with = "ts_milliseconds::deserialize")]
    pub updated_at: DateTime<Utc>,
    #[serde(deserialize_with = "ts_milliseconds::deserialize")]
    pub last_login_time: DateTime<Utc>,

    #[derivative(Debug = "ignore")]
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}
