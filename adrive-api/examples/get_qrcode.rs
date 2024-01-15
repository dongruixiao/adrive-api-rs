use std::error::Error;
use std::result::Result;

use reqwest::Client;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let sid = "1702702055e010e199a9e442fca180e9848860db93";
    let api_url: &str = "https://openapi.alipan.com";
    let path = format!("{}/oauth/qrcode/{}", api_url, sid);

    let res = client.get(Url::parse(&path)?).send().await?.bytes().await?;
    println!("{:#?}", res);
    Ok(())
}
