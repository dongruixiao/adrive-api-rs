pub mod auth;
use crate::constants::DOMAIN;
use async_trait::async_trait;
pub use auth::*;
use reqwest::Client;
use reqwest::Method;
use reqwest::Url;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::{error, result};

trait Response {}

#[async_trait]
pub trait Request: Sized + Serialize {
    const URI: &'static str;
    const METHOD: Method;
    type Response: DeserializeOwned;

    async fn dispatch(
        &self,
        reqwest_client: &Client,
    ) -> result::Result<Self::Response, Box<dyn error::Error>> {
        match Self::METHOD {
            Method::GET => self.get(reqwest_client).await,
            Method::POST => self.post(reqwest_client).await,
            _ => Err(format!("NotImplMethod: {}", Self::METHOD).into()),
        }
    }

    async fn post(
        &self,
        reqwest_client: &Client,
    ) -> result::Result<Self::Response, Box<dyn error::Error>> {
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

    async fn get(
        &self,
        reqwest_client: &Client,
    ) -> result::Result<Self::Response, Box<dyn error::Error>> {
        let path = Self::path_join()?;
        let resp = reqwest_client
            .get(path)
            .form(&self)
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
