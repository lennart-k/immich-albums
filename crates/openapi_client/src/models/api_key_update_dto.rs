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
pub struct ApiKeyUpdateDto {
    #[serde(rename = "name")]
    pub name: String,
}

impl ApiKeyUpdateDto {
    pub fn new(name: String) -> ApiKeyUpdateDto {
        ApiKeyUpdateDto {
            name,
        }
    }
}
