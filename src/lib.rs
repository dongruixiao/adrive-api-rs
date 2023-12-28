pub mod auth;
pub mod constants;
pub mod data_structures;

use data_structures::{
    GetDriveInfoRequest, GetDriveInfoResponse, GetSpaceInfoRequest, GetSpaceInfoResponse,
    GetUserInfoRequest, GetUserInfoResponse, Request,
};
use std::{error, result};

pub struct ADriveAPI<'a> {
    reqwest_client: reqwest::Client,
    auth: auth::Auth<'a>,
}

impl ADriveAPI<'_> {
    pub fn new() -> Self {
        ADriveAPI {
            reqwest_client: reqwest::Client::new(),
            auth: auth::Auth::new(),
        }
    }

    pub async fn user_info(&self) -> result::Result<GetUserInfoResponse, Box<dyn error::Error>> {
        let token = self.auth.refresh_if_needed().await?;
        let resp = GetUserInfoRequest {}
            .dispatch(&self.reqwest_client, None, Some(&token.access_token))
            .await?;
        Ok(resp)
    }

    pub async fn drive_info(&self) -> result::Result<GetDriveInfoResponse, Box<dyn error::Error>> {
        let token = self.auth.refresh_if_needed().await?;
        let resp = GetDriveInfoRequest {}
            .dispatch(&self.reqwest_client, None, Some(&token.access_token))
            .await?;
        Ok(resp)
    }

    pub async fn space_info(&self) -> result::Result<GetSpaceInfoResponse, Box<dyn error::Error>> {
        let token = self.auth.refresh_if_needed().await?;
        let resp = GetSpaceInfoRequest {}
            .dispatch(&self.reqwest_client, None, Some(&token.access_token))
            .await?;
        Ok(resp)
    }
}
