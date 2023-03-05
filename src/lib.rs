#![allow(dead_code)]

use std::fs::File;
use std::{cmp::min, io::SeekFrom};

use anyhow::{self, Ok};
pub mod objects;
// pub mod signin;
pub mod constants;
pub mod data_structures;
pub mod utils;

use base64::engine::{general_purpose, Engine};
use constants::{ADRIVE_BASE_URI, HTTPCLIENT};
use data_structures::files::create_file::{CompleteFileRequest, CompleteFileResponse};
use data_structures::files::{
    create_file::PartInfo, CreateDirRequest, CreateDirResponse, CreateFileRequest,
    CreateFileResponse, IfNameExists,
};
use data_structures::files::{MatchPreHashRequest, MatchPreHashResponse};
use objects::{
    Album, AlbumPayload, Capacity, CapacityPayload, Config, Credentials,
    Directory, FileExistsPayload, ListDirPayload, SafeBox, SafeBoxPayload,
    UserInfo, UserInfoPayload,
};
use reqwest::Url;
use serde::{de::DeserializeOwned, Serialize};
use sha1_smol::Sha1;
use std::io::{Read, Seek};

const ADRIVE_USER_INFO_API: &str = "adrive/v2/user/get";
const ADRIVE_CAPACITY_API: &str = "adrive/v1/user/driveCapacityDetails";
const ADRIVE_SAFEBOX_API: &str = "v2/sbox/get";
const ADRIVE_ALBUM_API: &str = "adrive/v1/user/albums_info";
const ADRIVE_LIST_DIR_API: &str = "adrive/v3/file/list";
const ADRIVE_FILE_EXISTS: &str = "adrive/v3/file/search";
const ADRIVE_CREATE_FOLDER: &str = "adrive/v2/file/createWithFolders";
const ADRIVE_COMPLETE_FILE: &str = "v2/file/complete";

const FILE_SIZE_HASH_LIMIT: u64 = 1024 * 1000;
const DEFAULT_PART_SIZE: u64 = 1024 * 1024 * 10; // 10MB

pub struct ADriveAPI {
    pub credentials: Credentials,
    pub config: Option<Config>,
}

impl ADriveAPI {
    pub fn new() -> ADriveAPI {
        Self {
            credentials: Credentials::new(),
            config: None,
        }
    }

    pub async fn user_info(&mut self) -> anyhow::Result<UserInfo> {
        let url = Self::join_url(ADRIVE_USER_INFO_API, None)?;
        let payload = UserInfoPayload {};
        let resp = self.request(url, payload).await?;
        Ok(resp)
    }

    pub async fn capacity(&mut self) -> anyhow::Result<Capacity> {
        let url = Self::join_url(ADRIVE_CAPACITY_API, None)?;
        let payload = CapacityPayload {};
        let resp = self.request(url, payload).await?;
        Ok(resp)
    }

    pub async fn safebox(&mut self) -> anyhow::Result<SafeBox> {
        let url = Self::join_url(ADRIVE_SAFEBOX_API, None)?;
        let payload = SafeBoxPayload {};
        let resp = self.request(url, payload).await?;
        Ok(resp)
    }

    pub async fn album(&mut self) -> anyhow::Result<Album> {
        let url = Self::join_url(ADRIVE_ALBUM_API, None)?;
        let payload = AlbumPayload {};
        let resp = self.request(url, payload).await?;
        Ok(resp)
    }

    pub async fn listdir(&mut self, parent_id: &str, limit: u32) -> anyhow::Result<Directory> {
        let url = Self::join_url(ADRIVE_LIST_DIR_API, None)?;
        let drive_id = self.credentials.drive_id.clone();
        let payload = ListDirPayload::new(&drive_id, parent_id, limit);
        let resp = self.request(url, payload).await?;
        Ok(resp)
    }

    // async fn drive_id(&mut self) -> anyhow::Result<String> {
    //     // if let Some(ref config) = self.config {
    //     //     return Ok(config.drive_id.clone());
    //     // } else {
    //     //     let user_info = self.user_info().await?;
    //     //     self.config = Some(Config {
    //     //         drive_id: user_info.default_drive_id.clone(),
    //     //     });
    //     //     return Ok(user_info.default_drive_id.clone());
    //     // }

    //     let res = self.config.get_or_insert_with(|| async {}.await);
    //     Ok(res.drive_id)
    // }

    /// multi files
    pub async fn exists(&mut self, parent_id: String, file_name: String) -> anyhow::Result<bool> {
        let url = Self::join_url(ADRIVE_FILE_EXISTS, None)?;
        let drive_id = self.credentials.drive_id.clone().to_string();
        let payload = FileExistsPayload::new(drive_id, parent_id, file_name);
        let resp: Directory = self.request(url, payload).await?;
        if resp.items.len() > 0 {
            return Ok(true);
        }
        Ok(false)
    }
    pub async fn create_dir(
        &mut self,
        parent_id: &str,
        name: &str,
        if_name_exists: Option<IfNameExists>,
    ) -> anyhow::Result<CreateDirResponse> {
        let url = Self::join_url(ADRIVE_CREATE_FOLDER, None)?;
        let drive_id = self.credentials.drive_id.to_owned();
        let payload: CreateDirRequest =
            CreateDirRequest::new(&drive_id, parent_id, name, if_name_exists);
        let resp: CreateDirResponse = self.request(url, payload).await?;
        Ok(resp)
    }

    fn get_part_info_list(size: u64) -> Vec<PartInfo> {
        let count = (size + DEFAULT_PART_SIZE - 1) / DEFAULT_PART_SIZE;
        (1..=count)
            .map(|index| PartInfo {
                part_number: index as u16,
                upload_url: None,
                internal_upload_url: None,
                content_type: None,
            })
            .collect()
    }

    fn get_content_hash(path: &str) -> anyhow::Result<String> {
        let mut file = File::open(path)?;
        let mut buf = vec![0u8; DEFAULT_PART_SIZE as usize];
        let mut s = Sha1::new();
        loop {
            let size = file.read(&mut buf)?;
            if size == 0 {
                break;
            }
            s.update(&mut buf[..size]);
        }
        let content_hash = s.digest().to_string();
        Ok(content_hash)
    }

    fn get_proof_code(name: &str, size: u64, token: &str) -> anyhow::Result<String> {
        if size <= 0 {
            return Ok(String::from(""));
        }
        let digest = md5::compute(token);
        let hex = format!("{:x}", digest);
        let uint = u64::from_str_radix(&hex[..16], 16)?;

        let start = uint % size;
        let end = min(start + 8, size);

        let mut buf = vec![0u8; (end - start) as usize];
        let mut file = File::open(name)?;
        file.seek(SeekFrom::Start(start))?;

        file.read_exact(&mut buf)?;
        Ok(general_purpose::STANDARD.encode(&buf))
    }

    fn get_pre_hash(name: &str) -> anyhow::Result<String> {
        let mut file = File::open(name)?;
        let mut buf = vec![0u8; 1024];
        let size = file.read(&mut buf)?;
        let data = &buf[..size];
        let mut s = Sha1::new();
        s.update(data);
        Ok(s.digest().to_string())
    }

    async fn single_part_upload(&mut self, url: &str, data: Vec<u8>) -> anyhow::Result<()> {
        let url = Url::parse(url)?;
        self.credentials.refresh_if_needed().await?;
        HTTPCLIENT
            .put(url)
            .body(data)
            .bearer_auth(&self.credentials.access_token)
            .send()
            .await?;
        Ok(())
    }

    async fn parallel_multipart_upload(&mut self, file: &mut File, part_info_list: Vec<PartInfo>) {
        todo!()
    }

    async fn complete_multipart_upload(
        &mut self,
        upload_id: &str,
        file_id: &str,
    ) -> anyhow::Result<CompleteFileResponse> {
        let url = Self::join_url(ADRIVE_COMPLETE_FILE, None)?;
        let drive_id = self.credentials.drive_id.to_owned();
        let payload: CompleteFileRequest = CompleteFileRequest {
            drive_id: &drive_id,
            upload_id,
            file_id,
        };
        let resp: CompleteFileResponse = self.request(url, payload).await?;
        Ok(resp)
    }

    async fn check_pre_hash(
        &mut self,
        url: Url,
        drive_id: &str,
        parent_id: &str,
        name: &str,
        if_name_exists: Option<IfNameExists>,
        size: u64,
    ) -> anyhow::Result<MatchPreHashResponse> {
        let pre_hash = Self::get_pre_hash(name)?;
        let part_info_list = Self::get_part_info_list(size);
        let payload = MatchPreHashRequest::new(
            drive_id,
            parent_id,
            name,
            if_name_exists,
            size,
            part_info_list,
            None,
            None,
            &pre_hash,
        );
        let resp: MatchPreHashResponse = self.request(url, payload).await?;
        Ok(resp)
    }

    async fn check_content_hash(
        &mut self,
        url: Url,
        drive_id: &str,
        parent_id: &str,
        name: &str,
        if_name_exists: Option<IfNameExists>,
        size: u64,
    ) -> anyhow::Result<CreateFileResponse> {
        let part_info_list = Self::get_part_info_list(size);
        let content_hash = Self::get_content_hash(name)?;
        let proof_code = Self::get_proof_code(name, size, &self.credentials.access_token)?;

        let payload = CreateFileRequest::new(
            &drive_id,
            parent_id,
            name,
            if_name_exists,
            size,
            part_info_list,
            None,
            None,
            &content_hash,
            None,
            &proof_code,
            None,
        );
        let resp: CreateFileResponse = self.request(url, payload).await?;
        Ok(resp)
    }

    // async fn create_file_response(
    //     &self,
    //     url: Url,
    //     drive_id: &str,
    //     parent_id: &str,
    //     name: &str,
    //     if_name_exists: Option<IfNameExists>,
    //     size: u64,
    // ) -> anyhow::Result<CreateFileResponse> {
    //     let resp = self
    //         .check_content_hash(url, drive_id, parent_id, name, if_name_exists, size)
    //         .await?;
    //     match &resp {
    //         CreateFileResponse::File {
    //             part_info_list,
    //             upload_id,
    //             rapid_upload,
    //             revision_id,
    //             location,
    //             _base,
    //         } => {
    //             todo!()
    //         }
    //         CreateFileResponse::RapidFile {
    //             upload_id,
    //             rapid_upload,
    //             revision_id,
    //             location,
    //             _base,
    //         } => {
    //             todo!()
    //         }
    //     }
    // }
    // pub async fn create_file(
    //     &mut self,
    //     parent_id: &str,
    //     name: &str,
    //     if_name_exists: Option<IfNameExists>,
    // ) -> anyhow::Result<CreateFileResponse> {
    //     /// 小于 limit 的文件直接走 createFileRequest 请求，之后返回 createFileResponse 来决定是否要上传，
    //     ///
    //     let url = Self::join_url(ADRIVE_CREATE_FOLDER, None)?;
    //     let drive_id = self.credentials.drive_id.to_string();

    //     let size = File::open(name)?.metadata()?.len();
    //     let part_info_list = Self::get_part_info_list(size);
    //     let content_hash = Self::get_content_hash(name)?;
    //     let proof_code = Self::get_proof_code(name, size, &self.credentials.access_token)?;

    //     if size < FILE_SIZE_HASH_LIMIT {
    //         let resp = self
    //             .create_file_response(url, &drive_id, parent_id, name, if_name_exists, size)
    //             .await?;
    //         return Ok(resp);
    //     }
    //     let resp = self
    //         .check_pre_hash(url, &drive_id, parent_id, name, if_name_exists, size)
    //         .await?;

    //     if resp.matched() {
    //         let resp = self
    //             .check_content_hash(url, &drive_id, parent_id, name, if_name_exists, size)
    //             .await?;
    //         return Ok(resp);
    //     }
    //     let resp = self
    //         .check_content_hash(url, &drive_id, parent_id, name, if_name_exists, size)
    //         .await?;
    //     Ok(resp)
    // }

    fn join_url(sub_url: &str, base_url: Option<&str>) -> anyhow::Result<Url> {
        let root_url: &str;
        if let Some(base) = base_url {
            root_url = base;
        } else {
            root_url = ADRIVE_BASE_URI;
        }
        let url = Url::parse(root_url)?.join(sub_url)?;
        Ok(url)
    }

    async fn request<'a, S, D>(&mut self, url: Url, payload: S) -> anyhow::Result<D>
    where
        S: Serialize,
        D: DeserializeOwned,
    {
        self.credentials.refresh_if_needed().await?;
        let resp = HTTPCLIENT
            .post(url)
            .json(&payload)
            .bearer_auth(&self.credentials.access_token)
            .send()
            .await?
            .json::<D>()
            .await?;

        Ok(resp)
    }
}

// #[tokio::test]
// async fn test_user_info() {
//     let mut api = ADriveAPI::new();
//     // let user = api.user_info().await.unwrap();
//     // let cap = api.capacity().await.unwrap();
//     // let album = api.album().await.unwrap();
//     // let sbox = api.safebox().await.unwrap();
//     // let list = api
//     //     .exists("root".to_string(), "4988_5_31INsypx64pzIbXI-a480.jpeg".to_string())
//     //     .await
//     //     .unwrap();
//     // println!("{:#?}", user);
//     // println!("{:#?}", cap);
//     // println!("{:#?}", album);
//     // println!("{:#?}", sbox);

//     // let resp = api.create_file("root", "Cargo.lock", None).await.unwrap();
//     // print!("{:#?}", resp);

//     // let ret = api.listdir("root", 100).await.unwrap();
//     // let ret = api
//     //     .create_file(
//     //         String::from("root"),
//     //         String::from("/Users/dongruixiao/Desktop/4988_5_31INsypx64pzIbXI-a480.jpeg"),
//     //         // String::from("/Users/dongruixiao/Desktop/a.txt"),
//     //     )
//     //     .await
//     //     .unwrap();
//     // println!("{:#?}", ret);
//     // println!("{}", ret);
//     // println!("{:#?}", folder.common.drive_id);
//     // println!("{:#?}", folder);
//     // println!("{:#?}", api.config);
// }
