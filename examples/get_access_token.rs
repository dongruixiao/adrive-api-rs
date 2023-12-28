use std::error::Error;
use std::result::Result;

use reqwest::Client;
use serde_json::json;
use url::Url;

struct AccessTokenRequest {}
enum AuthType {
    AuthorizationCode,
    RefreshToken,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let _sid = "1702702055e010e199a9e442fca180e9848860db93";
    let api_url: &str = "https://openapi.alipan.com";
    let path = format!("{}/oauth/access_token", api_url);

    let _payload = json!({
        "client_id": "",
        "client_secret": "",
        "grant_type":"authorization_code",
        "code": "",
        // "refresh_token": None
    });
    let res = client.get(Url::parse(&path)?).send().await?.bytes().await?;
    println!("{:#?}", res);
    Ok(())
}
