// use crate::apis::Metadata;
// use crate::data::file::create::CreateFileRequest;
// use crate::data::Request;

// pub trait Metadata {
//     const ADRIVE_BASE_API: &'static str = "";
//     fn signature(&self) {}
//     fn credentials(&self) {

//     }
// }

// #[async_trait::async_trait]
// pub trait File: Metadata {
//     async fn create_file(&self) {
//         let sign = self.signature();
//         let cred = self.credentials();
//         let resp = CreateFileRequest::new().send().await.unwrap();
//         todo!()
//     }
// }

pub struct File;

impl File {
    pub fn create_file() {
        todo!()
    }
}
