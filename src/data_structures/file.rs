use crate::data_structures::Request;
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub enum OrderBy {
    CreatedAt,
    #[default]
    UpdatedAt,
    Size,
    Name,
    NameEnhanced,
}

#[derive(Serialize, Default, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum SortBy {
    #[default]
    Desc,
    Asc,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "lowercase")]
pub enum FileType {
    Folder,
    File,
    #[default]
    #[serde(skip_deserializing)]
    All,
}

#[derive(Debug, Serialize, Default)]
pub struct GetFileListRequest<'a> {
    drive_id: &'a str,
    parent_file_id: &'a str,
    limit: Option<u32>, // 50..=100
    marker: Option<&'a str>,
    order_by: Option<OrderBy>,
    order_direction: Option<SortBy>,
    category: Option<&'a str>, // TODO
    r#type: Option<FileType>,
    video_thumbnail_time: Option<u32>,
    video_thumbnail_width: Option<u32>,
    video_thumbnail_height: Option<u32>,
    fields: Option<&'a str>, // TODO *
}

impl<'a> GetFileListRequest<'a> {
    pub fn new(drive_id: &'a str, parent_file_id: &'a str) -> Self {
        Self {
            drive_id,
            parent_file_id,
            ..Default::default()
        }
    }
}

impl Request for GetFileListRequest<'_> {
    const URI: &'static str = "/adrive/v1.0/openFile/list";
    const METHOD: reqwest::Method = Method::POST;
    type Response = GetFileListResponse;
}

#[derive(Debug, Deserialize)]
pub struct FileItem {
    pub drive_id: String,
    pub file_id: String,
    pub parent_file_id: String,
    pub name: String,
    pub size: Option<u64>,              // TODO folder don't have size
    pub file_extension: Option<String>, // TODO folder don't have file_extension
    pub content_hash: Option<String>,   // TODO folder don't have content_hash
    pub category: Option<String>,       // TODO folder don't have category
    pub r#type: FileType,
    pub thumbnail: Option<String>,
    pub url: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub play_cursor: Option<String>,
    pub video_media_metadata: Option<serde_json::Value>, // TODO
    pub video_preview_metadata: Option<String>,
    pub next_marker: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GetFileListResponse {
    pub items: Vec<FileItem>,
}
