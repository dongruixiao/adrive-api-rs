use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::{cmp, fs};

use base64::prelude::*;
use sha1_smol::Sha1;

pub fn ensure_dirs(dir: &str) -> crate::Result<PathBuf> {
    fs::create_dir_all(dir)?;
    let path = fs::canonicalize(dir)?;
    Ok(path)
}

pub fn get_proof_code(file: &mut fs::File, size: u64, token: &str) -> crate::Result<String> {
    file.seek(SeekFrom::Start(0))?;
    if size == 0 {
        return Ok(String::from(""));
    }
    let digest = md5::compute(token);
    let hex = format!("{:x}", digest);
    let uint = u64::from_str_radix(&hex[..16], 16)?;

    let start = uint % size;
    let end = cmp::min(start + 8, size);

    let mut buf = vec![0u8; (end - start) as usize];
    file.seek(SeekFrom::Start(start))?;

    file.read_exact(&mut buf)?;
    Ok(BASE64_STANDARD.encode(&buf))
}

pub fn get_content_hash(file: &mut fs::File) -> crate::Result<String> {
    file.seek(SeekFrom::Start(0))?;
    let mut hasher = Sha1::new();
    let mut buffer = vec![0u8; 10 * 1024];
    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        let data = &buffer[..count];
        hasher.update(data);
    }
    Ok(hasher.hexdigest().to_uppercase())
}

pub fn get_pre_hash(file: &mut fs::File) -> crate::Result<String> {
    // TODO 1024?
    file.seek(SeekFrom::Start(0))?;
    let mut buffer = vec![0u8; 1024];
    let count = file.read(&mut buffer)?;
    let data = &buffer[..count];
    let mut hasher = Sha1::new();
    hasher.update(data);
    Ok(hasher.hexdigest().to_uppercase())
}
