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
pub enum AssetJobName {
    #[serde(rename = "regenerate-thumbnail")]
    RegenerateThumbnail,
    #[serde(rename = "refresh-metadata")]
    RefreshMetadata,
    #[serde(rename = "transcode-video")]
    TranscodeVideo,

}

impl std::fmt::Display for AssetJobName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::RegenerateThumbnail => write!(f, "regenerate-thumbnail"),
            Self::RefreshMetadata => write!(f, "refresh-metadata"),
            Self::TranscodeVideo => write!(f, "transcode-video"),
        }
    }
}

impl Default for AssetJobName {
    fn default() -> AssetJobName {
        Self::RegenerateThumbnail
    }
}
