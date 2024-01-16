use crate::data_structures::{
    GetAccessTokenRequest2, GetAccessTokenRequest3, GetAccessTokenResponse, GetQRCodeRequest2,
    Request,
};
use crate::data_structures::{GetQRCodeStatusRequest, QRCodeStatus};
use std::path::PathBuf;
use std::{fs, thread, time};

use chrono::Utc;

pub struct Auth();

impl Auth {
    fn path() -> PathBuf {
        dirs::config_dir()
            .expect("no config dir detected")
            .join("adrive-api-rs/credentials")
    }

    fn dump(&self, token: &GetAccessTokenResponse) -> crate::Result<()> {
        let path = &Self::path();
        if !path.exists() {
            fs::create_dir_all(path.parent().expect("no parent dir detected"))?;
            fs::File::create(path)?;
        }

        let file = fs::File::create(path)?;
        serde_json::to_writer_pretty(file, token)?;
        Ok(())
    }

    pub fn load() -> crate::Result<GetAccessTokenResponse> {
        let file = fs::File::open(Self::path())?;
        let token: GetAccessTokenResponse = serde_json::from_reader(file)?;
        Ok(token)
    }

    pub async fn sign_in(&self) -> crate::Result<()> {
        let resp = GetQRCodeRequest2 {}.dispatch(None, None).await?;
        println!("### 🌟 请打开网页并扫码: {:#?}", resp.qr_code_url);
        let auth_code = loop {
            let resp = GetQRCodeStatusRequest { sid: &resp.sid }
                .dispatch(None, None)
                .await?;
            match resp.status {
                QRCodeStatus::WaitLogin => println!("### ⏳ 等待扫码登陆..."),
                QRCodeStatus::ScanSuccess => println!("### 🆗 扫码成功，等待确认..."),
                QRCodeStatus::LoginSuccess => {
                    println!("### ✅ 登陆成功");
                    break resp.auth_code;
                }
                QRCodeStatus::QRCodeExpired => {
                    println!("### ⛔️ 二维码已过期");
                    break None;
                }
            }
            thread::sleep(time::Duration::from_secs(1))
        };
        if auth_code.is_none() {
            return Ok(());
        }
        let resp = GetAccessTokenRequest2 {
            auth_code: &auth_code.unwrap(),
        }
        .dispatch(None, None)
        .await?;
        println!("### 👋");

        self.dump(&resp)?;
        Ok(())
    }

    pub async fn refresh_token(&self) -> crate::Result<GetAccessTokenResponse> {
        let token = Self::load()?;
        println!("{:#?}", token);
        let resp = GetAccessTokenRequest3 {
            refresh_token: &token.refresh_token,
        }
        .dispatch(None, None)
        .await?;
        self.dump(&resp)?;
        Ok(resp)
    }

    pub async fn refresh_if_needed(&self) -> crate::Result<GetAccessTokenResponse> {
        let token = Self::load()?;
        if Utc::now().timestamp() - token.time.timestamp() >= token.expires_in {
            let token = self.refresh_token().await?;
            Ok(token)
        } else {
            Ok(token)
        }
    }
}
