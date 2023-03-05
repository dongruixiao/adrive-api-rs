use lazy_static::lazy_static;
use reqwest::Client;

lazy_static! {
    pub static ref HTTPCLIENT: Client = Client::new();
}

pub const ADRIVE_PASSPORT_URI: &str = "https://passport.aliyundrive.com";
pub const ADRIVE_BASE_URI: &str = "https://api.aliyundrive.com";
