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
    All,
}

#[derive(Debug, Serialize)]
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

impl Request for GetFileListRequest<'_> {
    const URI: &'static str = "/adrive/v1.0/openFile/list";
    const METHOD: reqwest::Method = Method::POST;
    type Response = GetFileListResponse;
}
#[derive(Debug, Deserialize)]
pub struct GetFileListResponse {
    pub id: String,
    pub name: String,
    pub avatar: String,
    pub phone: Option<String>,
}
