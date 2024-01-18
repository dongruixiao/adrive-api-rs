use crate::data::Request;
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct GetUserInfoRequest;

impl Request for GetUserInfoRequest {
    const URI: &'static str = "/oauth/users/info";
    const METHOD: reqwest::Method = Method::GET;
    type Response = GetUserInfoResponse;
}
#[derive(Debug, Deserialize)]
pub struct GetUserInfoResponse {
    pub id: String,
    pub name: String,
    pub avatar: String,
    pub phone: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GetDriveInfoRequest;

impl Request for GetDriveInfoRequest {
    const URI: &'static str = "/adrive/v1.0/user/getDriveInfo";
    const METHOD: reqwest::Method = Method::POST;
    type Response = GetDriveInfoResponse;
}
#[derive(Debug, Deserialize)]
pub struct GetDriveInfoResponse {
    pub user_id: String,
    pub name: String,
    pub avatar: String,
    pub default_drive_id: String,
    pub resource_drive_id: Option<String>,
    pub backup_drive_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GetSpaceInfoRequest;

impl Request for GetSpaceInfoRequest {
    const URI: &'static str = "/adrive/v1.0/user/getSpaceInfo";
    const METHOD: reqwest::Method = Method::POST;
    type Response = GetSpaceInfoResponse;
}

#[derive(Debug, Deserialize)]
pub struct PersonalSpaceInfo {
    pub used_size: u64,
    pub total_size: u64,
}

#[derive(Debug, Deserialize)]
pub struct GetSpaceInfoResponse {
    pub personal_space_info: PersonalSpaceInfo,
}
