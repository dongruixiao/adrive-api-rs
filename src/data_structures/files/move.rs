use std::vec;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
struct RequestBody<'a> {
    drive_id: &'a str,
    file_id: &'a str,
    to_drive_id: &'a str,
    to_parent_file_id: &'a str,
}

#[derive(Serialize, Debug)]
struct RequestHeaders<'a> {
    #[serde(rename = "Content-Type")]
    content_type: &'a str,
}

#[derive(Serialize, Debug)]
struct Request<'a> {
    body: RequestBody<'a>,
    headers: RequestHeaders<'a>,
    id: &'a str,
    method: &'a str,
    url: &'a str,
}

#[derive(Serialize, Debug)]
pub struct MoveFileRequest<'a> {
    requests: Vec<Request<'a>>,
    resource: &'a str,
}

impl<'a> MoveFileRequest<'a> {
    pub fn new(
        drive_id: &'a str,
        file_id: &'a str,
        to_drive_id: &'a str,
        to_parent_file_id: &'a str,
    ) -> Self {
        let request = Request {
            body: RequestBody {
                drive_id,
                file_id,
                to_drive_id,
                to_parent_file_id,
            },
            headers: RequestHeaders {
                content_type: "application/json",
            },
            id: file_id,
            method: "POST",
            url: "/file/move",
        };
        Self {
            requests: vec![request],
            resource: "file",
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct ResponseBody {
    pub domain_id: String,
    pub drive_id: String,
    pub file_id: String,
}

#[derive(Deserialize, Debug)]
pub struct Response {
    pub body: ResponseBody,
    pub id: String,
    pub status: u16,
}

#[derive(Deserialize, Debug)]
pub struct MoveFileResponse {
    pub responses: Vec<Response>,
}
