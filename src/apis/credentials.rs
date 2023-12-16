use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, File};
use std::path::PathBuf;

use crate::data::{RefreshTokenRequest, RefreshTokenResponse, Request};

const CREDENTIALS_PATH: &str = "adrive-api-rs/credentials";
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
    pub fn new() -> Self {
        Self::load().unwrap()
    }

    pub fn path() -> PathBuf {
        dirs::config_dir()
            .expect("no config dir detected")
            .join(CREDENTIALS_PATH)
    }

    pub fn load() -> crate::Result<Self> {
        let file = File::open(Self::path())?;
        let credentials: Credentials = serde_json::from_reader(file)?;
        Ok(credentials)
    }

    pub fn dump(&self) -> crate::Result<()> {
        let path = &Self::path();
        if !path.exists() {
            create_dir_all(path.parent().expect("no parent dir detected"))?;
        }

        let file = File::create(path)?;
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }

    pub async fn refresh(&mut self, adrive: &crate::ADriveAPI) -> crate::Result<()> {
        let response = RefreshTokenRequest {
            refresh_token: &self.refresh_token,
        }
        .send::<RefreshTokenResponse>(adrive)
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

    pub async fn refresh_if_needed(&mut self, adrive: &crate::ADriveAPI) -> crate::Result<()> {
        if self.expire_time.timestamp() - Utc::now().timestamp()
            <= self.expire_in / EXPIRE_TIME_ZOOM_OUT
        {
            self.refresh(adrive).await?;
        }
        Ok(())
    }
}
