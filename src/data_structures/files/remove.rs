use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
struct Body<'a> {
    drive_id: &'a str,
    file_id: &'a str,
}

#[derive(Serialize, Debug)]
struct Headers<'a> {
    #[serde(rename = "Content-Type")]
    content_type: &'a str,
}

#[derive(Serialize, Debug)]
struct Request<'a> {
    body: Body<'a>,
    headers: Headers<'a>,
    id: &'a str,
    method: &'a str,
    url: &'a str,
}

#[derive(Serialize, Debug)]
pub struct RemoveFileRequest<'a> {
    requests: Vec<Request<'a>>,
    resource: &'a str,
}

impl<'a> RemoveFileRequest<'a> {
    pub fn new(drive_id: &'a str, file_ids: Vec<&'a str>) -> Self {
        let requests = file_ids
            .into_iter()
            .map(|file_id| Request {
                body: Body { drive_id, file_id },
                headers: Headers {
                    content_type: "application/json",
                },
                id: file_id,
                method: "POST",
                url: "/recyclebin/trash",
            })
            .collect();
        Self {
            requests,
            resource: "file",
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Response {
    pub id: String,
    pub status: u16,
}

#[derive(Deserialize, Debug)]
pub struct RemoveFileResponse {
    pub responses: Vec<Response>,
}
