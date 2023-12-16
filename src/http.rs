use lazy_static::lazy_static;
use reqwest::Client;

lazy_static! {
    pub static ref HTTPCLIENT: Client = Client::new();
}
