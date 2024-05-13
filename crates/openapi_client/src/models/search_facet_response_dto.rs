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
pub struct SearchFacetResponseDto {
    #[serde(rename = "counts")]
    pub counts: Vec<models::SearchFacetCountResponseDto>,
    #[serde(rename = "fieldName")]
    pub field_name: String,
}

impl SearchFacetResponseDto {
    pub fn new(counts: Vec<models::SearchFacetCountResponseDto>, field_name: String) -> SearchFacetResponseDto {
        SearchFacetResponseDto {
            counts,
            field_name,
        }
    }
}
