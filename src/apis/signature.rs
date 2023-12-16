pub struct Signature {
    device_id: Option<String>,
    user_id: Option<String>,
    app_id: Option<String>,
    nonce: i32,
}

impl Signature {
    pub fn new() -> Self {
        Self {
            device_id: None,
            user_id: None,
            app_id: None,
            nonce: i32::default(),
        }
    }
    pub fn sign(&mut self, adrive: &crate::ADriveAPI) {
        let device_id = if let Some(x) = self.device_id {
            x
        } else {
            String::from("aa")
        };
        // if let self.device_id = Some(x) {
        //     x
        // } else {
        //     12
        // }
    }
}
