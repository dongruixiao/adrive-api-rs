use reqwest::Client;
use reqwest::Url;
use serde_json::json;
use serde_json::Value;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let api_url: &str = "https://openapi.alipan.com";
    let url = format!("{}/oauth/authorize/qrcode", api_url);
    // let mut payload = HashMap::new();
    // payload.insert("client_id", "a3d0ef008fba45e8b7465f5e102628ee");
    // payload.insert("client_secret", "9db64416ca374bc5abfc5196e39ce8de");
    // payload.insert(
    //     "scope",
    //     vec![
    //         "user:base",
    //         "file:all:read",
    //         "file:all:write",
    //         "album:shared:read",
    //     ],
    // );
    let payload = json!({
        "client_id": "a3d0ef008fba45e8b7465f5e102628ee",
        "client_secret": "9db64416ca374bc5abfc5196e39ce8de",
        "scopes": [
            "user:base",
            "file:all:read",
            "file:all:write",
            // "album:shared:read",
        ],
        "width": 430,
        "height": 430,
    });

    let res = client
        .post(Url::parse(&url)?)
        .json(&payload)
        .send()
        .await?
        .json::<Value>()
        .await?;
    println!("{:#?}", res);
    Ok(())
}
