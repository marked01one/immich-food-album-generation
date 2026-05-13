use crate::utils::date_format;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum AssetType {
    IMAGE,
    VIDEO,
    AUDIO,
    OTHER,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum AssetVisibility {
    archive,
    timeline,
    hidden,
    locked,
}

// Follow the response schema: https://api.immich.app/endpoints/assets/getAssetInfo
#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct AssetResponse {
    pub checksum: String,

    #[serde(with = "date_format")]
    pub createdAt: DateTime<Utc>,
    #[serde(with = "date_format")]
    pub fileCreatedAt: DateTime<Utc>,
    #[serde(with = "date_format")]
    pub fileModifiedAt: DateTime<Utc>,

    pub deviceAssetId: String,
    pub deviceId: String,
    pub duplicateId: Option<String>,
    pub duration: String,

    pub hasMetadata: bool,
    pub height: Option<i64>,
    pub id: String,
    pub isArchived: bool,
    pub isEdited: bool,
    pub isFavorite: bool,
    pub isOffline: bool,
    pub isTrashed: bool,

    #[serde(with = "date_format")]
    pub localDateTime: DateTime<Utc>,

    pub originalFileName: String,
    pub originalMimeType: String,
    pub originalPath: String,
    pub ownerId: String,
    pub thumbhash: Option<String>,

    #[serde(rename = "type")]
    pub type_: AssetType,

    #[serde(with = "date_format")]
    pub updatedAt: DateTime<Utc>,

    pub visibility: AssetVisibility,
    pub width: Option<i64>,
}
