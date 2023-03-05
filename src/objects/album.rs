// Request URL: https://api.aliyundrive.com/adrive/v1/user/albums_info

use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Debug)]
pub struct AlbumPayload {}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AlbumData {
    pub drive_id: String,
    pub drive_name: String,
}

#[derive(Derivative, Deserialize)]
#[derivative(Debug)]
#[serde(rename_all = "camelCase")]
pub struct Album {
    pub data: AlbumData,

    #[derivative(Debug = "ignore")]
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}
