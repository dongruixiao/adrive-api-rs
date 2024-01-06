pub mod auth;
pub mod file;
pub mod user;
use crate::Result;
use async_trait::async_trait;
pub use auth::*;
pub use file::*;
use reqwest::{header::HeaderMap, Client, Method, Url};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::OnceLock;
pub use user::*;

pub static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

#[async_trait]
pub trait Request: Sized + Serialize {
    const DOMAIN: &'static str = "https://openapi.alipan.com";
    const URI: &'static str;
    const METHOD: Method;
    type Response: DeserializeOwned;

    fn reqwest_client() -> &'static Client {
        CLIENT.get_or_init(Client::new)
    }

    async fn dispatch(
        &self,
        headers: Option<HeaderMap>,
        token: Option<&str>,
    ) -> Result<Self::Response> {
        match Self::METHOD {
            Method::GET => self.get(headers, token).await,
            Method::POST => self.post(headers, token).await,
            _ => Err(format!("NotImplMethod: {}", Self::METHOD).into()),
        }
    }

    async fn post(
        &self,
        headers: Option<HeaderMap>,
        token: Option<&str>,
    ) -> Result<Self::Response> {
        let path = self.path_join()?;
        let resp = Self::reqwest_client()
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

    async fn get(&self, headers: Option<HeaderMap>, token: Option<&str>) -> Result<Self::Response> {
        let path = self.path_join()?;
        let resp = Self::reqwest_client()
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

    async fn get_original(
        &self,
        headers: Option<HeaderMap>,
        token: Option<&str>,
    ) -> Result<reqwest::Response> {
        let path = self.path_join()?;
        let resp = Self::reqwest_client()
            .get(path)
            .bearer_auth(token.unwrap_or_default())
            .headers(headers.unwrap_or_default())
            .send()
            .await?;
        Ok(resp)
    }

    fn path_join(&self) -> Result<Url> {
        let path = Url::parse(Self::DOMAIN)?.join(Self::URI)?;
        Ok(path)
    }
}
