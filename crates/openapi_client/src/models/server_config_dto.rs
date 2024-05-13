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
pub struct ServerConfigDto {
    #[serde(rename = "externalDomain")]
    pub external_domain: String,
    #[serde(rename = "isInitialized")]
    pub is_initialized: bool,
    #[serde(rename = "isOnboarded")]
    pub is_onboarded: bool,
    #[serde(rename = "loginPageMessage")]
    pub login_page_message: String,
    #[serde(rename = "oauthButtonText")]
    pub oauth_button_text: String,
    #[serde(rename = "trashDays")]
    pub trash_days: i32,
    #[serde(rename = "userDeleteDelay")]
    pub user_delete_delay: i32,
}

impl ServerConfigDto {
    pub fn new(external_domain: String, is_initialized: bool, is_onboarded: bool, login_page_message: String, oauth_button_text: String, trash_days: i32, user_delete_delay: i32) -> ServerConfigDto {
        ServerConfigDto {
            external_domain,
            is_initialized,
            is_onboarded,
            login_page_message,
            oauth_button_text,
            trash_days,
            user_delete_delay,
        }
    }
}
