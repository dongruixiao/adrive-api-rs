use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct StarredRequestBody<'a> {
    drive_id: &'a str,
    file_id: &'a str,
    starred: bool,
    custom_index_key: &'a str,
}

#[derive(Debug, Serialize)]
struct Headers<'a> {
    #[serde(rename = "Content-Type")]
    content_type: &'a str,
}

#[derive(Debug, Serialize)]
struct StarredRequest<'a> {
    body: StarredRequestBody<'a>,
    headers: Headers<'a>,
    id: &'a str,
    method: &'a str,
    url: &'a str,
}

#[derive(Debug, Serialize)]
pub struct StarredFileRequest<'a> {
    requests: Vec<StarredRequest<'a>>,
    resource: &'a str,
}

impl<'a> StarredFileRequest<'a> {
    pub fn new(drive_id: &'a str, file_id: &'a str, starred: bool) -> Self {
        let request = StarredRequest {
            body: StarredRequestBody {
                drive_id,
                file_id,
                starred,
                custom_index_key: starred.then(|| "starred_yes").unwrap_or(""),
            },
            headers: Headers {
                content_type: "application/json",
            },
            id: file_id,
            method: "PUT",
            url: "/file/update",
        };
        Self {
            requests: vec![request],
            resource: "file",
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct UserTag {
    channel: String,
    client: String,
    device_id: String,
    device_name: String,
}
#[derive(Deserialize, Debug)]
pub struct StarredResponseBody {
    user_meta: String,
    upload_id: String,
    hidden: bool,
    content_hash_name: String,
    parent_file_id: String,
    created_at: String,
    r#type: String,
    last_modifier_id: String,
    domain_id: String,
    last_modifier_name: String,
    last_modifier_type: String,
    content_type: String,
    starred: bool,
    updated_at: String,
    download_url: String,
    content_hash: String,
    revision_id: String,
    thumbnail: String,
    creator_type: String,
    drive_id: String,
    punish_flag: u32,
    revision_version: String,
    url: String,
    user_tags: UserTag,
    size: u64,
    crc64_hash: String,
    file_id: String,
    creator_id: String,
    name: String,
    creator_name: String,
    file_extension: String,
    category: String,
    encrypt_mode: String,
    status: String,
}

#[derive(Deserialize, Debug)]
pub struct StarredResponse {
    body: StarredResponseBody,
}

#[derive(Deserialize, Debug)]
pub struct StarredFileResponse {
    responses: Vec<StarredResponse>,
    id: String,
    status: u16,
}
