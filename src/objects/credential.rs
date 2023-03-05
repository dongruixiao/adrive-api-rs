use crate::constants::{ADRIVE_BASE_URI, HTTPCLIENT};
use crate::objects::{RefreshTokenRequest, RefreshTokenResponse};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const ADRIVE_CREDENTIALS_PATH: &str = "adrive-api-rs/credentials";
const ADRIVE_REFRESH_TOKEN_API: &str = "token/refresh";
const EXPIRE_TIME_ZOOM_OUT: i64 = 100;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Credentials {
    pub refresh_token: String,
    pub access_token: String,
    pub expire_time: DateTime<Utc>,
    pub expire_in: i64,
    pub drive_id: String,
    pub sbox_id: String,
}

impl Credentials {
    pub fn new() -> Credentials {
        Self::load().unwrap()
    }

    pub fn path() -> PathBuf {
        dirs::config_dir()
            .expect("no config dir detected")
            .join(ADRIVE_CREDENTIALS_PATH)
    }

    pub fn load() -> anyhow::Result<Self> {
        let file = std::fs::File::open(Self::path())?;
        let credentials: Credentials = serde_json::from_reader(file)?;
        Ok(credentials)
    }

    pub fn dump(&self) -> anyhow::Result<()> {
        let path = &Self::path();
        if !path.exists() {
            std::fs::create_dir_all(path.parent().expect("no parent dir detected"))?;
            std::fs::File::create(path)?;
        }

        let file = std::fs::File::create(path)?;
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }

    pub async fn refresh(&mut self) -> anyhow::Result<()> {
        let url = reqwest::Url::parse(ADRIVE_BASE_URI)?.join(ADRIVE_REFRESH_TOKEN_API)?;
        let request = RefreshTokenRequest {
            refresh_token: &self.refresh_token,
        };
        let response = HTTPCLIENT
            .post(url)
            .json(&request)
            .send()
            .await?
            .json::<RefreshTokenResponse>()
            .await?;

        self.refresh_token = response.refresh_token;
        self.access_token = response.access_token;
        self.expire_in = response.expires_in;
        self.expire_time = response.expire_time;
        self.drive_id = response.default_drive_id;
        self.sbox_id = response.default_sbox_drive_id;

        self.dump()?;
        Ok(())
    }

    pub async fn refresh_if_needed(&mut self) -> anyhow::Result<()> {
        if self.expire_time.timestamp() - Utc::now().timestamp()
            <= self.expire_in / EXPIRE_TIME_ZOOM_OUT
        {
            self.refresh().await?;
        }
        Ok(())
    }
}
