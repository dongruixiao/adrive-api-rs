# adrive-api-rs

Rust implementation of Aliyundrive API.

## Usage

build package

```shell

git clone git@github.com:dongruixiao/adrive-api-rs.git

cd adrive-api-rs

cargo build --debug

```

login

```shell

$ cargo run --bin sign
    Finished dev [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/sign`

### 🌟 请打开网页并扫码："https://openapi.alipan.com/oauth/qrcode/xxxxx"
### ⏳ 等待扫码登陆...
### ⏳ 等待扫码登陆...
### ⏳ 等待扫码登陆...
### 🆗 扫码成功，等待确认...
### ✅ 登陆成功
### 👋

```

example

```rust

use adrive_api_rs::ADriveAPI;
use adrive_api_rs::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let api = ADriveAPI::new();
    let drive_id = api.get_backup_drive_id().await?;

    let parent_id: &str = "root";
    let file_path = "/path/to/file";
    let resp = api.upload_file(&drive_id, parent_id, file_path).await?;
    println!("{:#?}", resp);
    Ok(())
}

```

## Others

I am a beginner in Rust, and this is my first project developed with Rust. It is currently usable. The progress is slow, and my learning pace is also slow, but I will continue to update it. Looking forward to your contribution to this project.
