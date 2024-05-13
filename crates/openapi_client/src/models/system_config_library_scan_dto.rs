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
pub struct SystemConfigLibraryScanDto {
    #[serde(rename = "cronExpression")]
    pub cron_expression: String,
    #[serde(rename = "enabled")]
    pub enabled: bool,
}

impl SystemConfigLibraryScanDto {
    pub fn new(cron_expression: String, enabled: bool) -> SystemConfigLibraryScanDto {
        SystemConfigLibraryScanDto {
            cron_expression,
            enabled,
        }
    }
}
