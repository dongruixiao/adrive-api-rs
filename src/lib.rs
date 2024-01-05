pub mod auth;
pub mod constants;
pub mod data_structures;

use data_structures::{
    BatchGetFileDetailByIdRequest, BatchGetFileDetailByIdResponse, FileSearchingRequest,
    FileSearchingResponse, GetDownloadUrlByIdRequest, GetDownloadUrlByIdResponse,
    GetDriveInfoRequest, GetDriveInfoResponse, GetFileDetailByIdRequest,
    GetFileDetailByPathRequest, GetFileDetailResponse, GetFileListRequest, GetFileListResponse,
    GetFileStarredListRequest, GetFileStarredListResponse, GetSpaceInfoRequest,
    GetSpaceInfoResponse, GetUserInfoRequest, GetUserInfoResponse, Request,
};
use std::{error, result};

type Result<T> = result::Result<T, Box<dyn error::Error>>;

pub struct ADriveAPI<'a> {
    auth: auth::Auth<'a>,
}

impl ADriveAPI<'_> {
    pub fn new() -> Self {
        Self {
            auth: auth::Auth::new(),
        }
    }

    pub async fn user_info(&self) -> Result<GetUserInfoResponse> {
        let token = self.auth.refresh_if_needed().await?;
        let resp = GetUserInfoRequest {}
            .dispatch(None, Some(&token.access_token))
            .await?;
        Ok(resp)
    }

    pub async fn drive_info(&self) -> Result<GetDriveInfoResponse> {
        let token = self.auth.refresh_if_needed().await?;
        let resp = GetDriveInfoRequest {}
            .dispatch(None, Some(&token.access_token))
            .await?;
        Ok(resp)
    }

    pub async fn space_info(&self) -> Result<GetSpaceInfoResponse> {
        let token: data_structures::GetAccessTokenResponse = self.auth.refresh_if_needed().await?;
        let resp = GetSpaceInfoRequest {}
            .dispatch(None, Some(&token.access_token))
            .await?;
        Ok(resp)
    }

    pub async fn get_default_drive_id(&self) -> Result<String> {
        Ok(self.drive_info().await?.default_drive_id)
    }

    pub async fn get_resource_drive_id(&self) -> Result<String> {
        Ok(self.drive_info().await?.resource_drive_id.unwrap())
    }

    pub async fn get_backup_drive_id(&self) -> Result<String> {
        Ok(self.drive_info().await?.backup_drive_id.unwrap())
    }

    pub async fn get_file_list(
        &self,
        drive_id: &str,
        parent_file_id: &str,
    ) -> Result<GetFileListResponse> {
        let token = self.auth.refresh_if_needed().await?;
        let resp = GetFileListRequest::new(drive_id, parent_file_id)
            .dispatch(None, Some(&token.access_token))
            .await?;
        Ok(resp)
    }

    pub async fn search_for_file(
        &mut self,
        drive_id: &str,
        query: &str,
    ) -> Result<FileSearchingResponse> {
        let token = self.auth.refresh_if_needed().await?;
        let resp = FileSearchingRequest::new(drive_id, Some(query))
            .dispatch(None, Some(&token.access_token))
            .await?;
        Ok(resp)
    }

    pub async fn get_starred_file_list(
        &self,
        drive_id: &str,
    ) -> Result<GetFileStarredListResponse> {
        let token = self.auth.refresh_if_needed().await?;
        let resp = GetFileStarredListRequest::new(drive_id)
            .dispatch(None, Some(&token.access_token))
            .await?;
        Ok(resp)
    }

    pub async fn get_file_detail_by_id(
        &self,
        drive_id: &str,
        file_id: &str,
    ) -> Result<GetFileDetailResponse> {
        let token = self.auth.refresh_if_needed().await?;
        GetFileDetailByIdRequest::new(drive_id, file_id)
            .dispatch(None, Some(&token.access_token))
            .await
    }

    pub async fn get_file_detail_by_path(
        &self,
        drive_id: &str,
        path: &str,
    ) -> Result<GetFileDetailResponse> {
        let token = self.auth.refresh_if_needed().await?;
        GetFileDetailByPathRequest::new(drive_id, path)
            .dispatch(None, Some(&token.access_token))
            .await
    }

    pub async fn batch_file_detail_by_id(
        &self,
        drive_ids: &[&str],
        file_ids: &[&str],
    ) -> Result<BatchGetFileDetailByIdResponse> {
        let token = self.auth.refresh_if_needed().await?;
        let mut file_list = Vec::new();
        let zipper = drive_ids.iter().zip(file_ids.iter());
        for (drive_id, file_id) in zipper {
            file_list.push(GetFileDetailByIdRequest::new(drive_id, file_id));
        }
        BatchGetFileDetailByIdRequest { file_list }
            .dispatch(None, Some(&token.access_token))
            .await
    }

    pub async fn get_download_url_by_file_id(
        &self,
        drive_id: &str,
        file_id: &str,
    ) -> Result<GetDownloadUrlByIdResponse> {
        let token = self.auth.refresh_if_needed().await?;
        GetDownloadUrlByIdRequest::new(drive_id, file_id)
            .dispatch(None, Some(&token.access_token))
            .await
    }
}
