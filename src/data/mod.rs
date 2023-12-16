pub mod credentials;
pub mod file;
pub mod info;
pub mod signature;
pub mod token;

pub use self::token::{RefreshTokenRequest, RefreshTokenResponse};


use crate::ADriveAPI;
use reqwest::Url;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt::Debug;

pub trait Response<'de>: Debug + Deserialize<'de> {
    fn raw() {}
}

#[async_trait::async_trait]
pub trait Request: Debug + Serialize {
    const API_BASE: &'static str = "https://api.aliyundrive.com";
    const API_PASSPORT: &'static str = "https://passport.aliyundrive.com";
    const API_PATH: &'static str;

    fn target() -> crate::Result<Url> {
        let dst = Url::parse(Self::API_BASE)?.join(Self::API_PATH)?;
        Ok(dst)
    }

    async fn send<T: DeserializeOwned>(&self, adrive: &ADriveAPI) -> crate::Result<T> {
        let dst = Self::target()?;

        let resp = adrive
            .client
            .post(dst)
            .json(&self)
            .send()
            .await?
            .json::<T>()
            .await?;
        Ok(resp)
    }
}
