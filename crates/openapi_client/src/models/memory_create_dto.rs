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
pub struct MemoryCreateDto {
    #[serde(rename = "assetIds", skip_serializing_if = "Option::is_none")]
    pub asset_ids: Option<Vec<uuid::Uuid>>,
    #[serde(rename = "data")]
    pub data: Box<models::OnThisDayDto>,
    #[serde(rename = "isSaved", skip_serializing_if = "Option::is_none")]
    pub is_saved: Option<bool>,
    #[serde(rename = "memoryAt")]
    pub memory_at: String,
    #[serde(rename = "seenAt", skip_serializing_if = "Option::is_none")]
    pub seen_at: Option<String>,
    #[serde(rename = "type")]
    pub r#type: models::MemoryType,
}

impl MemoryCreateDto {
    pub fn new(data: models::OnThisDayDto, memory_at: String, r#type: models::MemoryType) -> MemoryCreateDto {
        MemoryCreateDto {
            asset_ids: None,
            data: Box::new(data),
            is_saved: None,
            memory_at,
            seen_at: None,
            r#type,
        }
    }
}
