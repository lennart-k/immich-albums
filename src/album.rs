use chrono::{DateTime, Utc};
use openapi::models::UpdateAlbumDto;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct AlbumMetadata {
    pub name: String,
    pub description: Option<String>,
}

impl From<AlbumMetadata> for UpdateAlbumDto {
    fn from(value: AlbumMetadata) -> Self {
        Self {
            album_name: Some(value.name),
            description: value.description,
            ..Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AlbumInfo {
    pub metadata: AlbumMetadata,
    #[serde(default)]
    pub immich: AlbumFileImmich,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AlbumFileImmich {
    pub album_id: Option<Uuid>,
    pub updated_at: Option<DateTime<Utc>>,
}
