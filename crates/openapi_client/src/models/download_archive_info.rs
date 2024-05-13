/*
 * Immich
 *
 * Immich API
 *
 * The version of the OpenAPI document: 1.103.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DownloadArchiveInfo {
    #[serde(rename = "assetIds")]
    pub asset_ids: Vec<String>,
    #[serde(rename = "size")]
    pub size: i32,
}

impl DownloadArchiveInfo {
    pub fn new(asset_ids: Vec<String>, size: i32) -> DownloadArchiveInfo {
        DownloadArchiveInfo {
            asset_ids,
            size,
        }
    }
}
