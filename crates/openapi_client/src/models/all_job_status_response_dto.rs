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
pub struct AllJobStatusResponseDto {
    #[serde(rename = "backgroundTask")]
    pub background_task: Box<models::JobStatusDto>,
    #[serde(rename = "faceDetection")]
    pub face_detection: Box<models::JobStatusDto>,
    #[serde(rename = "facialRecognition")]
    pub facial_recognition: Box<models::JobStatusDto>,
    #[serde(rename = "library")]
    pub library: Box<models::JobStatusDto>,
    #[serde(rename = "metadataExtraction")]
    pub metadata_extraction: Box<models::JobStatusDto>,
    #[serde(rename = "migration")]
    pub migration: Box<models::JobStatusDto>,
    #[serde(rename = "notifications")]
    pub notifications: Box<models::JobStatusDto>,
    #[serde(rename = "search")]
    pub search: Box<models::JobStatusDto>,
    #[serde(rename = "sidecar")]
    pub sidecar: Box<models::JobStatusDto>,
    #[serde(rename = "smartSearch")]
    pub smart_search: Box<models::JobStatusDto>,
    #[serde(rename = "storageTemplateMigration")]
    pub storage_template_migration: Box<models::JobStatusDto>,
    #[serde(rename = "thumbnailGeneration")]
    pub thumbnail_generation: Box<models::JobStatusDto>,
    #[serde(rename = "videoConversion")]
    pub video_conversion: Box<models::JobStatusDto>,
}

impl AllJobStatusResponseDto {
    pub fn new(background_task: models::JobStatusDto, face_detection: models::JobStatusDto, facial_recognition: models::JobStatusDto, library: models::JobStatusDto, metadata_extraction: models::JobStatusDto, migration: models::JobStatusDto, notifications: models::JobStatusDto, search: models::JobStatusDto, sidecar: models::JobStatusDto, smart_search: models::JobStatusDto, storage_template_migration: models::JobStatusDto, thumbnail_generation: models::JobStatusDto, video_conversion: models::JobStatusDto) -> AllJobStatusResponseDto {
        AllJobStatusResponseDto {
            background_task: Box::new(background_task),
            face_detection: Box::new(face_detection),
            facial_recognition: Box::new(facial_recognition),
            library: Box::new(library),
            metadata_extraction: Box::new(metadata_extraction),
            migration: Box::new(migration),
            notifications: Box::new(notifications),
            search: Box::new(search),
            sidecar: Box::new(sidecar),
            smart_search: Box::new(smart_search),
            storage_template_migration: Box::new(storage_template_migration),
            thumbnail_generation: Box::new(thumbnail_generation),
            video_conversion: Box::new(video_conversion),
        }
    }
}
