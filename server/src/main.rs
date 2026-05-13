mod api;
mod shared;
mod utils;

use dotenvy::dotenv;

use api::responses::asset::{AssetResponse, AssetType};
use shared::pipeline::get_assets_from_album;
use std::collections::{HashMap, HashSet};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let album_id = "8d7beebc-5fbf-4418-b5a0-6ac9bc51d630";
    let album_id_private = "7b67ff8b-f28b-4c29-8f4f-899c67639f19";
    let response_bind = get_assets_from_album(album_id).await?;
    let response_private_bind = get_assets_from_album(album_id_private).await?;

    let mut response_map = response_bind
        .into_iter()
        .filter(|a| a.type_ == AssetType::IMAGE)
        .map(|a| (a.id.clone(), a))
        .collect::<HashMap<String, AssetResponse>>();

    let private_assets = response_private_bind
        .into_iter()
        .filter(|a| a.type_ == AssetType::IMAGE)
        .map(|a| a.id)
        .collect::<HashSet<String>>();

    let album_size_pre_filter = response_map.len() as i64;

    for pa in private_assets {
        response_map.remove(&pa);
    }

    let album_size_post_filter = response_map.len() as i64;

    println!("--------------------------------");
    println!("Images ingested:\t{album_size_pre_filter}");
    println!("Images to process:\t{album_size_post_filter}");

    Ok(())
}
