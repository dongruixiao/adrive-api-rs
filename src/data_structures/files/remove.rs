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
struct RemoveFileRequest<'a> {
    requests: Vec<Request<'a>>,
    resource: &'a str,
}

#[derive(Deserialize, Debug)]
struct Response {
    id: String,
    status: u16,
}

#[derive(Deserialize, Debug)]
struct RemoveFileResponse {
    responses: Vec<Response>,
}
