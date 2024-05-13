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
pub enum JobCommand {
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "pause")]
    Pause,
    #[serde(rename = "resume")]
    Resume,
    #[serde(rename = "empty")]
    Empty,
    #[serde(rename = "clear-failed")]
    ClearFailed,

}

impl std::fmt::Display for JobCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Start => write!(f, "start"),
            Self::Pause => write!(f, "pause"),
            Self::Resume => write!(f, "resume"),
            Self::Empty => write!(f, "empty"),
            Self::ClearFailed => write!(f, "clear-failed"),
        }
    }
}

impl Default for JobCommand {
    fn default() -> JobCommand {
        Self::Start
    }
}
