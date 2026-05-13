use std::error::Error;

use crate::api::responses::{album::AlbumResponse, asset::AssetResponse};

pub async fn get_asset(asset_id: &str) -> Result<AssetResponse, Box<dyn Error>> {
    let api_key = std::env::var("IMMICH_API_KEY").expect("IMMICH_API_KEY must be set");

    let immich_get_url = format!(
        "http://hydrogen:2283/api/assets/{asset_id}?apiKey={api_key}",
        asset_id = asset_id,
        api_key = api_key
    );

    let resp = reqwest::get(immich_get_url)
        .await?
        .json::<Option<AssetResponse>>()
        .await?
        .expect("Failed to get asset!");

    Ok(resp)
}

pub async fn get_assets_from_album(album_id: &str) -> Result<Vec<AssetResponse>, Box<dyn Error>> {
    let api_key = std::env::var("IMMICH_API_KEY").expect("IMMICH_API_KEY must be set");

    // Construct query URL.
    let url = &format!(
        "http://hydrogen:2283/api/albums/{album_id}?apiKey={api_key}",
        album_id = album_id,
        api_key = api_key
    );

    // Extract response.
    let resp = reqwest::get(url)
        .await?
        .json::<Option<AlbumResponse>>()
        .await?
        .map(|a| a.assets)
        .expect("Failed to get assets from album!");

    Ok(resp)
}
