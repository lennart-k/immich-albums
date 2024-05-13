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
pub struct AssetBulkDeleteDto {
    #[serde(rename = "force", skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    #[serde(rename = "ids")]
    pub ids: Vec<uuid::Uuid>,
}

impl AssetBulkDeleteDto {
    pub fn new(ids: Vec<uuid::Uuid>) -> AssetBulkDeleteDto {
        AssetBulkDeleteDto {
            force: None,
            ids,
        }
    }
}

