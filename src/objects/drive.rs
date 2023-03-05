// Request URL: https://api.aliyundrive.com/adrive/v1/user/driveCapacityDetails
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct CapacityPayload {}

#[derive(Deserialize, Debug)]
pub struct Capacity {
    pub drive_used_size: u64,
    pub drive_total_size: u64,
    pub default_drive_used_size: u64,
    pub album_drive_used_size: u64,
    pub share_album_drive_used_size: u64,
    pub note_drive_used_size: u64,
    pub sbox_drive_used_size: u64,
}
