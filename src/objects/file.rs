#![allow(unused)]

use crate::utils::deser::rfc3339_string_as_datetime;
use anyhow::Ok;
use chrono::{DateTime, Utc};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use sha1_smol::Sha1;
use std::collections::HashMap;
use std::fmt::{self, format};
use std::fs::File;
use std::io::{Read, Seek, Write};
use std::os::unix::prelude::MetadataExt;
use std::vec;
use std::{fs, hash};

use base64::{engine::general_purpose, Engine as _};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum FileType {
    Folder,
    File,
}

#[derive(Derivative, Deserialize)]
#[derivative(Debug)]
pub struct FileEntry {
    drive_id: String,
    file_id: String,
    name: String,
    parent_file_id: String,
    starred: bool, // 是否被收藏
    r#type: FileType,

    size: Option<u64>,

    #[serde(deserialize_with = "rfc3339_string_as_datetime")]
    created_at: DateTime<Utc>,
    #[serde(deserialize_with = "rfc3339_string_as_datetime")]
    updated_at: DateTime<Utc>,

    #[serde(flatten)]
    #[derivative(Debug = "ignore")]
    extra: HashMap<String, serde_json::Value>,
}

impl FileEntry {
    fn is_file(&self) -> bool {
        match self.r#type {
            FileType::File => true,
            _ => false,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct FileExistsPayload {
    drive_id: String,
    limit: u32,
    order_by: String,
    query: String,
}

impl FileExistsPayload {
    pub fn new(drive_id: String, parent_id: String, file_name: String) -> FileExistsPayload {
        Self {
            drive_id,
            limit: 100,
            order_by: String::from("name ASC"),
            query: format!(
                "parent_file_id = \"{}\" and (name = \"{}\")",
                parent_id, file_name
            ),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum CheckNameMode {
    AutoRename,
    Refuse,
    Ignore,
}

#[derive(Serialize, Debug)]
struct PartNumber {
    part_number: u32,
}

#[derive(Serialize, Debug)]
pub struct FileCreationPayload {
    /// necessary
    drive_id: String,
    name: String,
    check_name_mode: CheckNameMode,
    parent_file_id: String,
    r#type: FileType,
    /// optional
    #[serde(skip_serializing_if = "Option::is_none")]
    part_info_list: Option<Vec<PartNumber>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_scene: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_hash_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proof_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proof_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pre_hash: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct CommonCreation {
    pub drive_id: String,
    file_id: String,
    file_name: String,
    parent_file_id: String,
    r#type: FileType,
}

#[derive(Deserialize, Derivative)]
#[derivative(Debug)]
pub struct FolderCreation {
    #[serde(flatten)]
    pub common: CommonCreation,
    #[serde(flatten)]
    #[derivative(Debug = "ignore")]
    extra: HashMap<String, serde_json::Value>,
}

impl FileCreationPayload {
    pub fn new_folder(
        drive_id: String,
        parent_id: String,
        folder_name: String,
    ) -> FileCreationPayload {
        FileCreationPayload {
            drive_id,
            name: folder_name,
            check_name_mode: CheckNameMode::Refuse,
            parent_file_id: parent_id,
            r#type: FileType::Folder,
            part_info_list: None,
            size: None,
            create_scene: None,
            device_name: None,
            content_hash: None,
            content_hash_name: None,
            proof_code: None,
            proof_version: None,
            pre_hash: None,
        }
    }
    pub fn new_file(
        drive_id: String,
        parent_id: String,
        file_path: &str,
        access_token: String,
    ) -> FileCreationPayload {
        let size = fs::metadata(file_path).unwrap().len();
        let part_size = 1024 * 1024 * 10;
        let part_number = (size + part_size - 1) / part_size;

        let mut payload = FileCreationPayload {
            drive_id,
            name: file_path.to_string(),
            check_name_mode: CheckNameMode::AutoRename,
            parent_file_id: parent_id,
            r#type: FileType::File,
            part_info_list: Some(
                (1..part_number + 1)
                    .map(|part_index| PartNumber {
                        part_number: part_index as u32,
                    })
                    .collect(),
            ),
            size: Some(size),
            create_scene: Some(String::from("file_upload")), // TODO
            device_name: Some(String::from("")),
            content_hash: None,
            content_hash_name: None,
            proof_code: None,
            proof_version: None,
            pre_hash: None,
        };

        if size < 1024 * 1000 {
            let content_hash = Self::get_content_hash(file_path);
            let proof_code = Self::get_proof_code(file_path.to_string(), size, access_token);
            payload.content_hash = Some(content_hash);
            payload.device_name = Some(String::from("sha1"));
            payload.proof_code = Some(proof_code);
            payload.proof_version = Some(String::from("v1"));
        } else {
            let pre_hash = Self::get_pre_hash(file_path);
            payload.content_hash = Some(pre_hash);
        }

        payload
    }

    fn get_content_hash(file_path: &str) -> String {
        let mut file = fs::File::open(file_path).unwrap();
        let mut buffer = [0; 1024 * 1024 * 10];
        let mut hasher = Sha1::new();
        loop {
            let cksize = file.read(&mut buffer).unwrap();
            if cksize != 0 {
                hasher.update(&mut buffer[..cksize]);
            } else {
                break;
            }
        }
        hasher.digest().to_string()
    }

    fn get_pre_hash(file_path: &str) -> String {
        // 前 1024 个字节
        let mut file = fs::File::open(file_path).unwrap();
        let mut buffer = [0; 1024];
        let bytes_read = file.read(&mut buffer).unwrap();
        let data = &buffer[..bytes_read];
        let mut hasher = Sha1::new();
        hasher.update(data);
        hasher.digest().to_string()
    }

    fn get_proof_code(file_path: String, filesize: u64, access_token: String) -> String {
        if filesize <= 0 {
            return String::from("");
        }
        let digest = md5::compute(access_token);
        let digest = format!("{:x}", digest);
        let uint64 = &digest[..16];
        let uint64 = u64::from_str_radix(uint64, 16).unwrap();
        let start = uint64 % filesize;
        let mut end = start + 8;
        if end > filesize {
            end = filesize
        }
        let mut buffer = [0u8; 8];
        let mut file = fs::File::open(file_path).unwrap();
        file.seek(std::io::SeekFrom::Start(start)).unwrap();
        file.read_exact(&mut buffer[start as usize..end as usize])
            .unwrap();
        let s = end as usize - start as usize;
        general_purpose::STANDARD.encode(&buffer)
    }
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum CreateFileResponse {
    Folder {
        file_id: String,
        file_name: String,
        parent_file_id: String,
        r#type: FileType,
        drive_id: String,
        domain_id: String,
        encrypt_mode: String,
    },
    File {
        drive_id: String,
        file_id: String,
        file_name: String,
        parent_file_id: String,
        r#type: FileType,
        upload_id: String,
        rapid_upload: bool,
        revision_id: String,
        domain_id: String,
        encrypt_mode: String,
        location: String,
        part_info_list: Vec<PartInfo>,
    },
    RapidFile {
        drive_id: String,
        file_id: String,
        file_name: String,
        parent_file_id: String,
        r#type: FileType,
        upload_id: String,
        rapid_upload: bool,
        revision_id: String,
        domain_id: String,
        encrypt_mode: String,
        location: String,
    },
    PreHashMatched {
        part_number: u64,
        upload_url: String,
        internal_upload_url: String,
        content_type: String,
    },
}

#[derive(Deserialize, Debug)]
pub struct PartInfo {
    part_number: u64,
    upload_url: String,
    internal_upload_url: String,
    content_type: String,
}

#[derive(Deserialize, Derivative)]
#[derivative(Debug)]
pub struct FileCreation {
    #[serde(flatten)]
    pub common: CommonCreation,
    upload_id: String,
    rapid_upload: bool,
    revision_id: String,
    domain_id: String,
    encrypt_mode: String,
    location: String,
    part_info_list: Vec<PartInfo>,
    // #[serde(flatten)]
    // #[derivative(Debug = "ignore")]
    // extra: HashMap<String, serde_json::Value>,
}

#[derive(Deserialize, Debug)]
struct PreHashMatch {
    parent_file_id: String,
    file_name: String,
    pre_hash: String,
    code: String,
    message: String,
}
