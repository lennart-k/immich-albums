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
pub struct MemoryUpdateDto {
    #[serde(rename = "isSaved", skip_serializing_if = "Option::is_none")]
    pub is_saved: Option<bool>,
    #[serde(rename = "memoryAt", skip_serializing_if = "Option::is_none")]
    pub memory_at: Option<String>,
    #[serde(rename = "seenAt", skip_serializing_if = "Option::is_none")]
    pub seen_at: Option<String>,
}

impl MemoryUpdateDto {
    pub fn new() -> MemoryUpdateDto {
        MemoryUpdateDto {
            is_saved: None,
            memory_at: None,
            seen_at: None,
        }
    }
}

