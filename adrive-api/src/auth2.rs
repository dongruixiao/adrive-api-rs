use crate::data_structures::{
    GetAccessTokenRequest, GetAccessTokenResponse, GetQRCodeImageRequest, GetQRCodeSID, Request,
};
use crate::data_structures::{GetQRCodeRequest, GetQRCodeStatusRequest, QRCodeStatus};
use std::path::PathBuf;
use std::{fs, thread, time};

use chrono::Utc;
use reqwest::header::HeaderMap;

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
        let sid = GetQRCodeSID {}.dispatch(None, None).await?;
        println!("{:#?}", sid);
        // let mut headers = HeaderMap::new();
        // headers.insert("Content-Type", "image/jpeg".parse()?);
        // let resp = GetQRCodeImageRequest { sid: &sid }
        //     .dispatch(Some(headers), None)
        //     .await?;
        // println!("{:#?}", resp);
        // let auth_code = loop {
        //     let resp = GetQRCodeStatusRequest { sid: &resp.sid }
        //         .dispatch(None, None)
        //         .await?;
        //     match resp.status {
        //         QRCodeStatus::WaitLogin => println!("等待扫码登陆..."),
        //         QRCodeStatus::ScanSuccess => println!("扫码成功，等待确认..."),
        //         QRCodeStatus::LoginSuccess => {
        //             println!("登陆成功");
        //             break resp.auth_code;
        //         }
        //         QRCodeStatus::QRCodeExpired => {
        //             println!("二维码已过期");
        //             break None;
        //         }
        //     }
        //     thread::sleep(time::Duration::from_secs(1))
        // };
        // if auth_code.is_none() {
        //     return Ok(());
        // }
        // let resp = GetAccessTokenRequest::new(
        //     self.client_id,
        //     self.client_secret,
        //     Some(&auth_code.unwrap()),
        //     None,
        // )
        // .dispatch(None, None)
        // .await?;
        // println!("{:#?}", resp);

        // println!("########################################");
        // println!("### 登陆成功：{}", resp.access_token);
        // println!("########################################");

        // self.dump(&resp)?;
        Ok(())
    }

    async fn refresh_token(&self) -> crate::Result<GetAccessTokenResponse> {
        // let token = Self::load()?;
        // let resp = GetAccessTokenRequest::new(
        //     self.client_id,
        //     self.client_secret,
        //     None,
        //     Some(&token.refresh_token),
        // )
        // .dispatch(None, None)
        // .await?;
        // self.dump(&resp)?;
        // Ok(resp)
        todo!()
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
