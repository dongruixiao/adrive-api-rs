pub mod auth;
pub mod file;
pub mod user;
use crate::constants::DOMAIN;
use async_trait::async_trait;
pub use auth::*;
pub use file::*;
use reqwest::{header::HeaderMap, Client, Method, Url};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::{error, result};
pub use user::*;

trait Response {}

#[async_trait]
pub trait Request: Sized + Serialize {
    const URI: &'static str;
    const METHOD: Method;
    type Response: DeserializeOwned;

    async fn dispatch(
        &self,
        reqwest_client: &Client,
        headers: Option<HeaderMap>,
        token: Option<&str>,
    ) -> result::Result<Self::Response, Box<dyn error::Error>> {
        match Self::METHOD {
            Method::GET => self.get(reqwest_client, headers, token).await,
            Method::POST => self.post(reqwest_client, headers, token).await,
            _ => Err(format!("NotImplMethod: {}", Self::METHOD).into()),
        }
    }

    async fn post(
        &self,
        reqwest_client: &Client,
        headers: Option<HeaderMap>,
        token: Option<&str>,
    ) -> result::Result<Self::Response, Box<dyn error::Error>> {
        let path = self.path_join()?;
        let resp = reqwest_client
            .post(path)
            .bearer_auth(token.unwrap_or_default())
            .headers(headers.unwrap_or_default())
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
        headers: Option<HeaderMap>,
        token: Option<&str>,
    ) -> result::Result<Self::Response, Box<dyn error::Error>> {
        let path = self.path_join()?;
        let resp = reqwest_client
            .get(path)
            .bearer_auth(token.unwrap_or_default())
            .headers(headers.unwrap_or_default())
            .form(&self)
            .send()
            .await?
            .json::<Self::Response>()
            .await?;
        Ok(resp)
    }

    fn path_join(&self) -> result::Result<Url, Box<dyn error::Error>> {
        let path = Url::parse(DOMAIN)?.join(Self::URI)?;
        Ok(path)
    }
}
