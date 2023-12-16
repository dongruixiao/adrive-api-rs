use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct SafeBox {
    id: String,
}
#[derive(Deserialize, Debug)]
struct SafeBoxPayload {
    idx: String,
    #[serde(flatten)]
    safebox: SafeBox,
}

fn main() {
    let s = r#"{"idx": "aaa", "id": "xxx"}"#;
    let r: SafeBoxPayload = serde_json::from_str(&s).unwrap();
    println!("{:#?}", r);
    println!("{:#?}", r.safebox.id);
}
