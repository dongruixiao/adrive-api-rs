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
    video_thumbnail_time: Option<u32>,  // ms
    video_thumbnail_width: Option<u32>, // px
    image_thumbnail_width: Option<u32>, // px
    fields: Option<&'a str>,            // TODO *
}

impl<'a> GetFileListRequest<'a> {
    pub fn new(drive_id: &'a str, parent_file_id: &'a str) -> Self {
        Self {
            drive_id,
            parent_file_id,
            video_thumbnail_time: Some(120000), // ms
            video_thumbnail_width: Some(480),   // px
            image_thumbnail_width: Some(480),   // px
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
pub struct FileListingItem {
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
    pub items: Vec<FileListingItem>,
}

#[derive(Debug, Serialize, Default)]
pub struct FileSearchingRequest<'a> {
    drive_id: &'a str,
    limit: Option<u32>,
    marker: Option<&'a str>,
    /*
    查询语句，样例：
    固定目录搜索，只搜索一级 parent_file_id = '123'
    精确查询 name = '123'
    模糊匹配 name match "123"
    搜索指定后缀文件 file_extension = 'apk'
    范围查询 created_at < "2019-01-14T00:00:00"
    复合查询：
    type = 'folder' or name = '123'
    parent_file_id = 'root' and name = '123' and category = 'video'
    */
    query: Option<&'a str>,
    /*
    created_at ASC | DESC
    updated_at ASC | DESC
    name ASC | DESC
    size ASC | DESC
    */
    order_by: Option<OrderBy>,
    video_thumbnail_time: Option<u32>,
    video_thumbnail_width: Option<u32>,
    image_thumbnail_width: Option<u32>,
    return_total_count: Option<bool>,
}

impl<'a> FileSearchingRequest<'a> {
    pub fn new(drive_id: &'a str, query: Option<&'a str>) -> Self {
        Self {
            drive_id,
            query,
            video_thumbnail_time: Some(120000), // ms
            video_thumbnail_width: Some(480),   // px
            image_thumbnail_width: Some(480),   // px
            ..Default::default()
        }
    }
}

impl Request for FileSearchingRequest<'_> {
    const URI: &'static str = "/adrive/v1.0/openFile/search";
    const METHOD: reqwest::Method = Method::POST;
    type Response = FileSearchingResponse;
}

#[derive(Debug, Deserialize)]
pub struct FileSearchingItem {
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
    pub next_marker: Option<String>,
    pub total_count: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct FileSearchingResponse {
    pub items: Vec<FileSearchingItem>,
}

#[derive(Debug, Serialize, Default)]
pub struct GetFileStarredListRequest<'a> {
    drive_id: &'a str,
    limit: Option<u32>,
    marker: Option<&'a str>,
    r#type: Option<FileType>,
    order_by: Option<OrderBy>,
    order_direction: Option<SortBy>,
    video_thumbnail_time: Option<u32>,
    video_thumbnail_width: Option<u32>,
    image_thumbnail_width: Option<u32>,
}

impl<'a> GetFileStarredListRequest<'a> {
    pub fn new(drive_id: &'a str) -> Self {
        Self {
            drive_id,
            video_thumbnail_time: Some(120000), // ms
            video_thumbnail_width: Some(480),   // px
            image_thumbnail_width: Some(480),   // px
            ..Default::default()
        }
    }
}

impl Request for GetFileStarredListRequest<'_> {
    const URI: &'static str = "/adrive/v1.0/openFile/starredList";
    const METHOD: reqwest::Method = Method::POST;
    type Response = GetFileStarredListResponse;
}

#[derive(Debug, Deserialize)]
pub struct FileStarredItem {
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
    pub next_marker: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GetFileStarredListResponse {
    pub items: Vec<FileStarredItem>,
}

#[derive(Debug, Serialize, Default)]
pub struct GetFileDetailByIdRequest<'a> {
    drive_id: &'a str,
    file_id: &'a str,
    video_thumbnail_time: Option<u32>,
    video_thumbnail_width: Option<u32>,
    image_thumbnail_width: Option<u32>,
    fields: Option<&'a str>, // *
}

impl<'a> GetFileDetailByIdRequest<'a> {
    pub fn new(drive_id: &'a str, file_id: &'a str) -> Self {
        Self {
            drive_id,
            file_id,
            video_thumbnail_time: Some(120000), // ms
            video_thumbnail_width: Some(480),   // px
            image_thumbnail_width: Some(480),   // px
            ..Default::default()
        }
    }
}

impl Request for GetFileDetailByIdRequest<'_> {
    const URI: &'static str = "/adrive/v1.0/openFile/get";
    const METHOD: reqwest::Method = Method::POST;
    type Response = GetFileDetailResponse;
}

#[derive(Debug, Deserialize)]
pub struct GetFileDetailResponse {
    pub drive_id: String,
    pub file_id: String,
    pub parent_file_id: String,
    pub name: String,
    pub size: u64,
    pub file_extension: String,
    pub content_hash: String,
    pub category: String,
    pub r#type: FileType,
    pub thumbnail: String,
    pub url: String,
    pub created_at: String,
    pub updated_at: String,
    pub video_media_metadata: serde_json::Value,
}

#[derive(Debug, Serialize, Default)]
pub struct GetFileDetailByPathRequest<'a> {
    drive_id: &'a str,
    file_path: &'a str,
}

impl<'a> GetFileDetailByPathRequest<'a> {
    pub fn new(drive_id: &'a str, file_path: &'a str) -> Self {
        Self {
            drive_id,
            file_path,
        }
    }
}

impl Request for GetFileDetailByPathRequest<'_> {
    const URI: &'static str = "/adrive/v1.0/openFile/get_by_path";
    const METHOD: reqwest::Method = Method::POST;
    type Response = GetFileDetailResponse;
}

#[derive(Debug, Serialize, Default)]
pub struct BatchGetFileDetailByIdRequest<'a> {
    pub file_list: Vec<GetFileDetailByIdRequest<'a>>,
}

impl Request for BatchGetFileDetailByIdRequest<'_> {
    const URI: &'static str = "/adrive/v1.0/openFile/batch/get";
    const METHOD: reqwest::Method = Method::POST;
    type Response = BatchGetFileDetailByIdResponse;
}

#[derive(Debug, Deserialize)]
pub struct BatchGetFileDetailByIdResponse {
    items: Vec<GetFileDetailResponse>,
}
