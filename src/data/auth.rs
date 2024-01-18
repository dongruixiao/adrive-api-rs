use super::Request;
use chrono::{DateTime, Utc};
use reqwest::{Method, Url};
use serde::{Deserialize, Serialize};

const SELF_HOSTED_SERVER: &'static str = "https://adrive-sign-in.onrender.com";

#[derive(Debug, Serialize)]
pub struct GetQRCodeRequest<'a> {
    client_id: &'a str,
    client_secret: &'a str,
    scopes: Vec<&'a str>,
    width: u32,
    height: u32,
}

impl<'a> GetQRCodeRequest<'a> {
    pub fn new(client_id: &'a str, client_secret: &'a str) -> Self {
        let scopes = vec![
            "user:base",
            "file:all:read",
            "file:all:write",
            // "album:shared:read",
        ];
        Self {
            client_id,
            client_secret,
            scopes,
            width: 430,
            height: 430,
        }
    }
}

impl Request for GetQRCodeRequest<'_> {
    const URI: &'static str = "/oauth/authorize/qrcode";
    const METHOD: Method = Method::POST;
    type Response = GetQRCodeResponse;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetQRCodeResponse {
    #[serde(rename = "qrCodeUrl")]
    pub qr_code_url: String,
    pub sid: String,
}

#[derive(Debug, Serialize)]
pub struct GetQRCodeImageRequest<'a> {
    #[serde(skip_serializing)]
    pub sid: &'a str,
}

impl Request for GetQRCodeImageRequest<'_> {
    const URI: &'static str = "/oauth/qrcode/{sid}";
    const METHOD: Method = Method::GET;
    type Response = GetQRCodeImageResponse;

    fn path_join(&self) -> crate::Result<Url> {
        let uri = Self::URI.replace("{sid}", self.sid);
        let path = Url::parse(Self::DOMAIN)?.join(&uri)?;
        Ok(path)
    }
}

#[derive(Debug, Deserialize)]
pub struct GetQRCodeImageResponse {}

#[derive(Debug, Deserialize)]
pub enum QRCodeStatus {
    WaitLogin,
    ScanSuccess,
    LoginSuccess,
    QRCodeExpired,
}

impl From<&str> for QRCodeStatus {
    fn from(value: &str) -> Self {
        match value {
            "WaitLogin" => QRCodeStatus::WaitLogin,
            "ScanSuccess" => QRCodeStatus::ScanSuccess,
            "LoginSuccess" => QRCodeStatus::LoginSuccess,
            "QRCodeExpired" => QRCodeStatus::QRCodeExpired,
            _ => todo!(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct GetQRCodeStatusRequest<'a> {
    #[serde(skip_serializing)]
    pub sid: &'a str,
}

impl Request for GetQRCodeStatusRequest<'_> {
    const URI: &'static str = "/oauth/qrcode/{sid}/status";
    const METHOD: Method = Method::GET;
    type Response = GetQRCodeStatusResponse;

    fn path_join(&self) -> crate::Result<Url> {
        let uri = Self::URI.replace("{sid}", self.sid);
        let path = Url::parse(Self::DOMAIN)?.join(&uri)?;
        Ok(path)
    }
}
#[derive(Debug, Deserialize)]
pub struct GetQRCodeStatusResponse {
    pub status: QRCodeStatus,
    #[serde(rename = "authCode")]
    pub auth_code: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GrantType {
    AuthorizationCode,
    RefreshToken,
}

#[derive(Debug, Serialize)]
pub struct GetAccessTokenRequest<'a> {
    client_id: &'a str,
    client_secret: &'a str,
    grant_type: GrantType,
    code: Option<&'a str>,
    refresh_token: Option<&'a str>,
}

impl<'a> GetAccessTokenRequest<'a> {
    pub fn new(
        client_id: &'a str,
        client_secret: &'a str,
        code: Option<&'a str>,
        refresh_token: Option<&'a str>,
    ) -> Self {
        if refresh_token.is_some() {
            Self {
                client_id,
                client_secret,
                code: None,
                grant_type: GrantType::RefreshToken,
                refresh_token,
            }
        } else if code.is_some() {
            Self {
                client_id,
                client_secret,
                code,
                grant_type: GrantType::AuthorizationCode,
                refresh_token: None,
            }
        } else {
            panic!("code or refresh_token must be provided");
        }
    }
}

impl Request for GetAccessTokenRequest<'_> {
    const URI: &'static str = "/oauth/access_token";
    const METHOD: Method = Method::POST;
    type Response = GetAccessTokenResponse;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetAccessTokenResponse {
    pub token_type: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    #[serde(default = "Utc::now")]
    pub time: DateTime<Utc>,
    // pub code: String,
    // pub message: String,
}

#[derive(Debug, Serialize)]
pub struct GetQRCodeRequest2;

impl Request for GetQRCodeRequest2 {
    const DOMAIN: &'static str = SELF_HOSTED_SERVER;
    const URI: &'static str = "/sid";
    const METHOD: Method = Method::GET;
    type Response = GetQRCodeResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAccessTokenRequest2 {
    pub auth_code: String,
}

impl Request for GetAccessTokenRequest2 {
    const DOMAIN: &'static str = SELF_HOSTED_SERVER;
    const URI: &'static str = "/token";
    const METHOD: Method = Method::POST;
    type Response = GetAccessTokenResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAccessTokenRequest3 {
    pub refresh_token: String,
}

impl Request for GetAccessTokenRequest3 {
    const DOMAIN: &'static str = SELF_HOSTED_SERVER;
    const URI: &'static str = "/refresh_token";
    const METHOD: Method = Method::POST;
    type Response = GetAccessTokenResponse;
}
