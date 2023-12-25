use std::error::Error;
use std::result::Result;

use futures_util::TryFutureExt;
use reqwest::Client;
use serde_json::Value;
use url::Url;

enum QRCodeStatus {
    WaitLogin,
    ScanSuccess,
    LoginSuccess,
    QRCodeExpired,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let sid = "1702702055e010e199a9e442fca180e9848860db93";
    let api_url: &str = "https://openapi.alipan.com";
    let path = format!("{}/oauth/qrcode/{}/status", api_url, sid);

    let res = client
        .get(Url::parse(&path)?)
        .send()
        .await?
        .json::<Value>()
        .await?;
    println!("{:#?}", res);
    Ok(())
}
