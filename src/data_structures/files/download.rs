use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct DownloadFileRequest<'a> {
    drive_id: &'a str,
    file_id: &'a str,
}

impl<'a> DownloadFileRequest<'a> {
    pub fn new(drive_id: &'a str, file_id: &'a str) -> Self {
        Self { drive_id, file_id }
    }
}

#[derive(Deserialize, Debug)]
pub struct DownloadFileResponse {
    pub domain_id: String,
    pub drive_id: String,
    pub file_id: String,
    pub revision_id: String,
    pub method: String,
    pub url: String,
    pub internal_url: String,
    pub expiration: DateTime<Utc>,
    pub size: u64,
    pub crc64_hash: String,
    pub content_hash: String,
    pub content_hash_name: String,
}
