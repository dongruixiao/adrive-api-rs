// Request URL: https://api.aliyundrive.com/v2/sbox/get

use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Debug)]
pub struct SafeBoxPayload {}

#[derive(Derivative, Deserialize)]
#[derivative(Debug)]
pub struct SafeBox {
    pub drive_id: String,
    pub sbox_used_size: u64, // bytes
    pub sbox_real_used_size: u64,
    pub sbox_total_size: u64,

    #[derivative(Debug = "ignore")]
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}
