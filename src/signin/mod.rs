#![allow(dead_code)]

mod structure;

use crate::constant::{QRCODE_EXPIRED_TIME, QRCODE_QUERY_INTERVAL};
pub use crate::qrcode_sign_in::structure::SignedInToken;
use crate::qrcode_sign_in::structure::{
    BizExt, GenerateResponse, QRCodeStatus, QRCodeStatusError, QueryResponse,
};
use log;
use qr2term;
use reqwest::Client;
use std::{collections::HashMap, error, result, thread, time};

async fn query_until_comfirmed(
    client: &Client,
    payload: &HashMap<String, String>,
) -> result::Result<BizExt, Box<dyn error::Error>> {
    for _ in 0..QRCODE_EXPIRED_TIME / QRCODE_QUERY_INTERVAL {
        thread::sleep(time::Duration::from_secs(QRCODE_QUERY_INTERVAL));
        let response = QueryResponse::new(client, payload).await?;
        match response.get_status() {
            QRCodeStatus::New => {
                log::info!("Scan the QR code to log in...");
                continue;
            }
            QRCodeStatus::Scanned => {
                log::info!("QR code has been scanned, please confirm.");
                continue;
            }
            QRCodeStatus::Expired => {
                log::error!("QR code has expired, please try again.");
                return Err(QRCodeStatusError {
                    status: QRCodeStatus::Expired,
                    message: String::from("expired operation"),
                }
                .into());
            }
            QRCodeStatus::Confirmed => {
                if let Some(biz_ext) = response.get_biz_ext() {
                    log::info!("Signed in successfully.");
                    return BizExt::new(&biz_ext);
                } else {
                    log::error!("QR code has been confirmed, but it is not returned correctly.");
                    return Err(QRCodeStatusError {
                        status: QRCodeStatus::Confirmed,
                        message: String::from("not returned correctly"),
                    }
                    .into());
                }
            }
            QRCodeStatus::Unknown => {
                log::warn!("Unknown QR code status, please try again and confirm.");
                return Err(QRCodeStatusError {
                    status: QRCodeStatus::Unknown,
                    message: String::from("try again"),
                }
                .into());
            }
        }
    }
    log::error!("QR code has expired and the maximum waiting time has been reached.");
    Err(QRCodeStatusError {
        status: QRCodeStatus::Expired,
        message: String::from("maximum waiting time has been reached"),
    }
    .into())
}

pub async fn sign_in(client: &Client) -> result::Result<SignedInToken, Box<dyn error::Error>> {
    let resp = GenerateResponse::new(client).await?;
    qr2term::print_qr(&resp.get_code_content())?;
    let ref payload = resp.get_query_payload();
    let ext = query_until_comfirmed(client, payload).await?;
    Ok(ext.get_token())
}
