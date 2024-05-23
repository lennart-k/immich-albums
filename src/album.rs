use openapi::models::UpdateAlbumDto;
use serde::{Deserialize, Serialize};

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
