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
pub struct AssetBulkUploadCheckDto {
    #[serde(rename = "assets")]
    pub assets: Vec<models::AssetBulkUploadCheckItem>,
}

impl AssetBulkUploadCheckDto {
    pub fn new(assets: Vec<models::AssetBulkUploadCheckItem>) -> AssetBulkUploadCheckDto {
        AssetBulkUploadCheckDto {
            assets,
        }
    }
}

