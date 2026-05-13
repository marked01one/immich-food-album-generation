use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::api::responses::asset::{AssetOrder, AssetResponse};
use crate::api::responses::user::UserResponse;
use crate::utils::date_format;

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum AlbumUserRoleEnum {
    editor,
    viewer,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlbumUserResponse {
    pub role: AlbumUserRoleEnum,
    pub user: UserResponse,
}

// Follow the response schema: https://api.immich.app/endpoints/albums/getAlbumInfo
#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct AlbumResponse {
    pub albumName: String,
    pub albumThumbnailAssetId: Option<String>,
    pub albumUsers: Vec<AlbumUserResponse>,
    pub assetCount: i64,
    pub assets: Vec<AssetResponse>,

    #[serde(with = "date_format")]
    pub createdAt: DateTime<Utc>,

    pub description: String,
    pub hasSharedLink: bool,
    pub id: String,
    pub isActivityEnabled: bool,
    pub order: AssetOrder,
    pub owner: UserResponse,
    pub ownerId: String,
    pub shared: bool,

    #[serde(with = "date_format")]
    pub updatedAt: DateTime<Utc>,

    #[serde(with = "date_format")]
    pub lastModifiedAssetTimestamp: DateTime<Utc>,
}
