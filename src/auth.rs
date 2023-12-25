use crate::data_structures::GetQRCodeRequest;
use crate::data_structures::Request;
use reqwest::Client;
use std::{error, result};
pub struct Auth<'a> {
    reqwest_client: Client,
    client_id: &'a str,
    client_secret: &'a str,
}

impl Auth<'_> {
    pub fn new() -> Self {
        Self {
            reqwest_client: Client::new(),
            client_id: "a3d0ef008fba45e8b7465f5e102628ee",
            client_secret: "9db64416ca374bc5abfc5196e39ce8de",
        }
    }

    pub async fn sign_in(&self) -> result::Result<(), Box<dyn error::Error>> {
        let req: GetQRCodeRequest<'_> = GetQRCodeRequest::new(self.client_id, self.client_secret);
        let resp = req.dispatch(&self.reqwest_client).await?;
        println!("{:#?}", resp);
        Ok(())
    }

    fn refresh_token(&self) {}
    fn is_expired(&self) {}
}
