use crate::data_structures::{GetAccessTokenRequest, GetAccessTokenResponse, Request};
use crate::data_structures::{GetQRCodeRequest, GetQRCodeStatusRequest, QRCodeStatus};
use std::path::PathBuf;
use std::{thread, time};

use chrono::Utc;
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
        let resp = GetQRCodeRequest::new(self.client_id, self.client_secret)
            .dispatch(&self.reqwest_client, None, None)
            .await?;

        println!("########################################");
        println!("### 浏览器扫码登陆: {}", resp.qr_code_url);
        println!("########################################");

        let auth_code = loop {
            let resp = GetQRCodeStatusRequest { sid: &resp.sid }
                .dispatch(&self.reqwest_client, None, None)
                .await?;
            match resp.status {
                QRCodeStatus::WaitLogin => println!("等待扫码登陆..."),
                QRCodeStatus::ScanSuccess => println!("扫码成功，等待确认..."),
                QRCodeStatus::LoginSuccess => {
                    println!("登陆成功");
                    break resp.auth_code;
                }
                QRCodeStatus::QRCodeExpired => {
                    println!("二维码已过期");
                    break None;
                }
            }
            thread::sleep(time::Duration::from_secs(1))
        };
        if auth_code.is_none() {
            return Ok(());
        }
        let resp = GetAccessTokenRequest::new(
            self.client_id,
            self.client_secret,
            Some(&auth_code.unwrap()),
            None,
        )
        .dispatch(&self.reqwest_client, None, None)
        .await?;
        println!("{:#?}", resp);

        println!("########################################");
        println!("### 登陆成功: {}", resp.access_token);
        println!("########################################");

        self.dump(&resp)?;
        Ok(())
    }

    async fn refresh_token(&self) -> result::Result<GetAccessTokenResponse, Box<dyn error::Error>> {
        let token = Self::load()?;
        let resp = GetAccessTokenRequest::new(
            self.client_id,
            self.client_secret,
            None,
            Some(&token.refresh_token),
        )
        .dispatch(&self.reqwest_client, None, None)
        .await?;
        self.dump(&resp)?;
        Ok(resp)
    }

    pub async fn refresh_if_needed(
        &self,
    ) -> result::Result<GetAccessTokenResponse, Box<dyn error::Error>> {
        let token = Self::load()?;
        if Utc::now().timestamp() - token.time.timestamp() >= token.expires_in {
            let token = self.refresh_token().await?;
            Ok(token)
        } else {
            Ok(token)
        }
    }

    pub fn path() -> PathBuf {
        dirs::config_dir()
            .expect("no config dir detected")
            .join("adrive-api-rs/credentials")
    }

    pub fn dump(
        &self,
        token: &GetAccessTokenResponse,
    ) -> result::Result<(), Box<dyn error::Error>> {
        let path = &Self::path();
        if !path.exists() {
            std::fs::create_dir_all(path.parent().expect("no parent dir detected"))?;
            std::fs::File::create(path)?;
        }

        let file = std::fs::File::create(path)?;
        serde_json::to_writer_pretty(file, token)?;
        Ok(())
    }

    pub fn load() -> result::Result<GetAccessTokenResponse, Box<dyn error::Error>> {
        let file = std::fs::File::open(Self::path())?;
        let token: GetAccessTokenResponse = serde_json::from_reader(file)?;
        Ok(token)
    }
}
