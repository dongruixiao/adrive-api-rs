use crate::data_structures::Request;
use serde::{Deserialize, Serialize};

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
    type Response = GetQRCodeResponse;
}

#[derive(Debug, Deserialize)]
pub struct GetQRCodeResponse {
    #[serde(rename = "qrCodeUrl")]
    pub qr_code_url: String,
    pub sid: String,
}

#[derive(Debug, Serialize)]
pub struct GetQRCodeImageRequest {}

#[derive(Debug, Deserialize)]
pub struct GetQRCodeImageResponse {}

#[derive(Debug, Deserialize)]
pub enum QRCodeStatus {
    WaitLogin,
    ScanSuccess,
    LoginSuccess,
    QRCodeExpired,
}

#[derive(Debug, Serialize)]
pub struct GetQRCodeStatusRequest {}

#[derive(Debug, Deserialize)]
pub struct GetQRCodeStatusResponse {
    pub status: QRCodeStatus,
    #[serde(rename = "camelCase")]
    pub auth_code: Option<String>,
}

#[derive(Debug, Serialize)]
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

#[derive(Debug, Deserialize)]
pub struct GetAccessTokenResponse {
    pub token_type: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u32,
}
