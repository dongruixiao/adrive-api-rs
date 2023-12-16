// use crate::data::file::{FileType, IfNameExists};
// use crate::data::{Request, Response};
// use serde::{Deserialize, Serialize};

// #[derive(Serialize, Debug)]
// pub struct CreateFileRequest<'a> {
//     size: u64,
//     part_info_list: Vec<PartInfo>,
//     create_scene: &'a str,
//     device_name: &'a str,
//     content_hash: &'a str,
//     content_hash_name: &'a str,
//     proof_code: &'a str,
//     proof_version: &'a str,

//     #[serde(flatten)]
//     nested: NestedInRequest<'a>,
// }

// pub struct CreateFileResponse;

// #[derive(Serialize, Debug)]
// pub struct CreateDirRequest<'a> {
//     #[serde(flatten)]
//     nested: NestedInRequest<'a>,
// }

// pub struct CreateDirResponse;

// impl Request for CreateFileRequest<'_> {
//     const API_PATH: &'static str = "";
// }
// impl Response for CreateFileResponse {}
// impl Request for CreateDirRequest<'_> {
//     const API_PATH: &'static str = "";
// }

// impl CreateFileRequest<'_> {
//     pub fn new() -> Self {
//         todo!()
//     }
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct PartInfo {
//     pub part_number: u16,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub upload_url: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub internal_upload_url: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub content_type: Option<String>,
// }

// #[derive(Serialize, Debug)]
// struct NestedInRequest<'a> {
//     drive_id: &'a str,
//     name: &'a str,
//     #[serde(rename = "parent_file_id")]
//     parent_id: &'a str,
//     #[serde(rename = "check_name_mode")]
//     #[serde(default)]
//     if_name_exists: IfNameExists,
//     r#type: FileType,
// }
