use std::collections::HashMap;
use std::{error, fmt, result};

use serde::{Deserialize, Serialize};

use crate::constants::{ADRIVE_PASSPORT_URI, QRCODE_GENERATE_API, QRCODE_QUERY_API};

#[derive(Debug)]
pub struct SignedInToken {
    pub refresh_token: String,
    pub access_token: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct GenerateData {
    pub t: u64,
    pub code_content: String,
    pub ck: String,
    pub result_code: u32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct GenerateContent {
    pub data: GenerateData,
    pub status: u32,
    pub success: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct GenerateResponse {
    pub content: GenerateContent,
    pub has_error: bool,
}

impl GenerateResponse {
    pub async fn new(http_client: &reqwest::Client) -> result::Result<Self, Box<dyn error::Error>> {
        let url = reqwest::Url::parse(ADRIVE_PASSPORT_URI)?.join(QRCODE_GENERATE_API)?;
        let resp = http_client.get(url).send().await?.json::<Self>().await?;
        Ok(resp)
    }

    pub fn get_code_content(&self) -> String {
        self.content.data.code_content.to_string()
    }

    pub fn get_query_payload(&self) -> HashMap<String, String> {
        let mut payload = HashMap::new();
        payload.insert("t".to_string(), self.content.data.t.to_string());
        payload.insert("ck".to_string(), self.content.data.ck.to_string());
        payload
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct QueryData {
    pub qr_code_status: String,
    pub result_code: u32,
    pub biz_ext: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct QueryContent {
    pub data: QueryData,
    pub status: u32,
    pub success: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct QueryResponse {
    pub content: QueryContent,
    pub has_error: bool,
}

impl QueryResponse {
    pub async fn new(
        http_client: &reqwest::Client,
        payload: &HashMap<String, String>,
    ) -> result::Result<Self, Box<dyn error::Error>> {
        let url = reqwest::Url::parse(ADRIVE_PASSPORT_URI)?.join(QRCODE_QUERY_API)?;
        let resp = http_client
            .post(url)
            .form(payload)
            .send()
            .await?
            .json::<Self>()
            .await?;
        Ok(resp)
    }

    pub fn get_status(&self) -> QRCodeStatus {
        let status = &self.content.data.qr_code_status;
        match status.as_str() {
            "NEW" => QRCodeStatus::New,
            "SCANED" => QRCodeStatus::Scanned,
            "EXPIRED" => QRCodeStatus::Expired,
            "CONFIRMED" => QRCodeStatus::Confirmed,
            _ => QRCodeStatus::Unknown,
        }
    }

    pub fn get_biz_ext(&self) -> Option<String> {
        self.content.data.biz_ext.to_owned()
    }
}

#[derive(Debug)]
pub enum QRCodeStatus {
    New,
    Scanned,
    Expired,
    Confirmed,
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
struct PdsLoginResult {
    access_token: String,
    refresh_token: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct BizExt {
    pds_login_result: PdsLoginResult,
}

impl BizExt {
    pub fn new(base64_string: &String) -> result::Result<Self, Box<dyn error::Error>> {
        let result = base64::decode(base64_string.as_str())?;
        let result = result.iter().map(|&c| c as char).collect::<String>();
        let biz_ext = serde_json::from_str::<Self>(&result)?;
        Ok(biz_ext)
    }

    pub fn get_token(&self) -> SignedInToken {
        SignedInToken {
            refresh_token: self.pds_login_result.refresh_token.to_string(),
            access_token: self.pds_login_result.access_token.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct QRCodeStatusError {
    pub status: QRCodeStatus,
    pub message: String,
}

impl std::error::Error for QRCodeStatusError {}

impl fmt::Display for QRCodeStatusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, status: {:?}.",
            self.message.to_string(),
            self.status,
        )
    }
}
