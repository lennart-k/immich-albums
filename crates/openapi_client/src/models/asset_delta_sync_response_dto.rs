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
pub struct AssetDeltaSyncResponseDto {
    #[serde(rename = "deleted")]
    pub deleted: Vec<String>,
    #[serde(rename = "needsFullSync")]
    pub needs_full_sync: bool,
    #[serde(rename = "upserted")]
    pub upserted: Vec<models::AssetResponseDto>,
}

impl AssetDeltaSyncResponseDto {
    pub fn new(deleted: Vec<String>, needs_full_sync: bool, upserted: Vec<models::AssetResponseDto>) -> AssetDeltaSyncResponseDto {
        AssetDeltaSyncResponseDto {
            deleted,
            needs_full_sync,
            upserted,
        }
    }
}
