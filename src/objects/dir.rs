#![allow(unused)]

use crate::objects::{FileEntry, OrderBy, SortBy};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Debug)]
pub struct ListDirPayload<'a> {
    drive_id: &'a str,
    parent_file_id: &'a str,
    limit: u32,
    all: bool,
    url_expire_sec: u32,
    image_thumbnail_process: &'a str,
    image_url_process: &'a str,
    video_thumbnail_process: &'a str,
    fields: &'a str,
    order_by: OrderBy,
    order_direction: SortBy,
    marker: Option<&'a str>,
}

#[derive(Deserialize, Debug)]
pub struct Directory {
    pub items: Vec<FileEntry>,
    pub next_marker: String,
}

impl ListDirPayload<'_> {
    pub fn new<'a>(drive_id: &'a str, parent_file_id: &'a str, limit: u32) -> ListDirPayload<'a> {
        ListDirPayload {
            drive_id,
            parent_file_id,
            limit,
            all: Default::default(),
            url_expire_sec: 14400,
            image_thumbnail_process: "image/resize,w_256/format,jpeg",
            image_url_process: "image/resize,w_1920/format,jpeg/interlace,1",
            video_thumbnail_process: "video/snapshot,t_1000,f_jpg,ar_auto,w_256",
            fields: "*",
            order_by: OrderBy::default(),
            order_direction: SortBy::default(),
            marker: None,
        }
    }
}
