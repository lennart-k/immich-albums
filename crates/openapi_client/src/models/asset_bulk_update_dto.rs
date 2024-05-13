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
pub struct AssetBulkUpdateDto {
    #[serde(rename = "dateTimeOriginal", skip_serializing_if = "Option::is_none")]
    pub date_time_original: Option<String>,
    #[serde(rename = "ids")]
    pub ids: Vec<uuid::Uuid>,
    #[serde(rename = "isArchived", skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    #[serde(rename = "isFavorite", skip_serializing_if = "Option::is_none")]
    pub is_favorite: Option<bool>,
    #[serde(rename = "latitude", skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    #[serde(rename = "longitude", skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    #[serde(rename = "removeParent", skip_serializing_if = "Option::is_none")]
    pub remove_parent: Option<bool>,
    #[serde(rename = "stackParentId", skip_serializing_if = "Option::is_none")]
    pub stack_parent_id: Option<uuid::Uuid>,
}

impl AssetBulkUpdateDto {
    pub fn new(ids: Vec<uuid::Uuid>) -> AssetBulkUpdateDto {
        AssetBulkUpdateDto {
            date_time_original: None,
            ids,
            is_archived: None,
            is_favorite: None,
            latitude: None,
            longitude: None,
            remove_parent: None,
            stack_parent_id: None,
        }
    }
}

