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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ThumbnailFormat {
    #[serde(rename = "JPEG")]
    Jpeg,
    #[serde(rename = "WEBP")]
    Webp,

}

impl std::fmt::Display for ThumbnailFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Jpeg => write!(f, "JPEG"),
            Self::Webp => write!(f, "WEBP"),
        }
    }
}

impl Default for ThumbnailFormat {
    fn default() -> ThumbnailFormat {
        Self::Jpeg
    }
}
