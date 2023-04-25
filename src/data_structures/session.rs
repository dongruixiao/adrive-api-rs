use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateSessionRequest<'a> {
    device_name: &'a str,
    model_name: &'a str,
    pub_key: &'a str,
}

impl<'a> CreateSessionRequest<'a> {
    pub fn new(pub_key: &'a str) -> Self {
        Self {
            device_name: "Chrome浏览器",
            model_name: "Mac OS网页版",
            pub_key,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct RenewSessionRequest;

#[derive(Debug, Deserialize)]
pub struct SessionResponse {
    result: bool,
    success: bool,
    code: Option<String>,
    message: Option<String>,
}
