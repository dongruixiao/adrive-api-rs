mod auth;
mod error;
mod file;
mod user;
use crate::{constants, Result};
use async_trait::async_trait;
pub(crate) use auth::*;
pub(crate) use file::*;
use reqwest::StatusCode;
use reqwest::{header::HeaderMap, Client, Method, Url};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::OnceLock;
pub(crate) use user::*;

pub(crate) static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

#[async_trait]
pub(crate) trait Request: Sized + Serialize {
    const DOMAIN: &'static str = constants::ADRIVE_OPENAPI_DOMAIN;
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
            _ => unimplemented!(),
        }
    }

    async fn raise_for_status(&self, resp: reqwest::Response) -> Result<Self::Response> {
        match resp.error_for_status_ref() {
            Ok(_) => Ok(resp.json::<Self::Response>().await?),
            Err(err) => match err.status() {
                Some(StatusCode::CONFLICT) => Ok(resp.json::<Self::Response>().await?),
                Some(
                    StatusCode::BAD_REQUEST
                    | StatusCode::FORBIDDEN
                    | StatusCode::NOT_FOUND
                    | StatusCode::UNAUTHORIZED
                    | StatusCode::TOO_MANY_REQUESTS,
                ) => {
                    let err = resp.json::<error::ErrorResponse>().await?;
                    Err(err.into())
                }
                Some(_) => Err(err.into()),
                None => Err(err.into()),
            },
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
            .await?;
        self.raise_for_status(resp).await
    }

    async fn get(&self, headers: Option<HeaderMap>, token: Option<&str>) -> Result<Self::Response> {
        let path = self.path_join()?;
        let resp = Self::reqwest_client()
            .get(path)
            .bearer_auth(token.unwrap_or_default())
            .headers(headers.unwrap_or_default())
            .form(&self)
            .send()
            .await?;
        self.raise_for_status(resp).await
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

    async fn put_original(
        &self,
        headers: Option<HeaderMap>,
        token: Option<&str>,
        data: Vec<u8>,
    ) -> Result<reqwest::Response> {
        let path = self.path_join()?;
        let resp = Self::reqwest_client()
            .put(path)
            .body(data)
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
