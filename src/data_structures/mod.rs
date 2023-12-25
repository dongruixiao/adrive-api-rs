pub mod auth;
use crate::constants::DOMAIN;
use async_trait::async_trait;
pub use auth::*;
use reqwest::Client;
use reqwest::Url;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::{error, result};

trait Response {}

#[async_trait]
pub trait Request {
    type Response: DeserializeOwned;
    const URI: &'static str;

    async fn dispatch(
        &self,
        reqwest_client: &Client,
    ) -> result::Result<Self::Response, Box<dyn error::Error>>
    where
        Self: Sized + Serialize,
    {
        let path = Self::path_join()?;
        let resp = reqwest_client
            .post(path)
            .json(&self)
            .send()
            .await?
            .json::<Self::Response>()
            .await?;
        Ok(resp)
    }

    fn path_join() -> result::Result<Url, Box<dyn error::Error>> {
        let path = Url::parse(DOMAIN)?.join(Self::URI)?;
        Ok(path)
    }
}
