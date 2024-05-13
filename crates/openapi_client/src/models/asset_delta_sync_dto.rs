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
pub struct AssetDeltaSyncDto {
    #[serde(rename = "updatedAfter")]
    pub updated_after: String,
    #[serde(rename = "userIds")]
    pub user_ids: Vec<uuid::Uuid>,
}

impl AssetDeltaSyncDto {
    pub fn new(updated_after: String, user_ids: Vec<uuid::Uuid>) -> AssetDeltaSyncDto {
        AssetDeltaSyncDto {
            updated_after,
            user_ids,
        }
    }
}
