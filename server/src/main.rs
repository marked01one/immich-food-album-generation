mod api;
mod shared;
mod utils;

use dotenvy::dotenv;

use api::responses::asset::{AssetResponse, AssetType};
use shared::pipeline::get_assets_from_album;
use std::collections::{HashMap, HashSet};

use crate::shared::{model::Model, pipeline::push_assets_to_album};

const SOURCE_ALBUM_ID: &str = "8d7beebc-5fbf-4418-b5a0-6ac9bc51d630";
const SOURCE_ALBUM_ID_PRIVATE: &str = "7b67ff8b-f28b-4c29-8f4f-899c67639f19";

const DESTINATION_ALBUM_ID: &str = "66ee6684-7c29-4e5f-a82f-13eaaf4d3a40";

const THRESHOLD: f64 = 0.5;
const LABELS: &[&str] = &[
    "not_food",
    "italian_food",
    "japanese_food",
    "fast_food",
    "meat",
    "seafood",
    "soup",
    "salad",
    "dessert",
    "rice",
    "eggs",
];

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let response_bind = get_assets_from_album(SOURCE_ALBUM_ID).await?;
    let response_private_bind = get_assets_from_album(SOURCE_ALBUM_ID_PRIVATE).await?;

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

    println!("--------------------------------");
    let model_file = format!("../models/resnet_food_epoch_20.pt");

    let model = Model::new(model_file, LABELS.iter().map(|&l| l.to_string()).collect());

    let food_images_uuids = response_map
        .iter()
        .filter(|(_, asset)| {
            let preds = model.predict_from_file(&asset.originalPath).expect(
                &format!("Failed to process asset with path: {}", asset.originalPath).to_string(),
            );
            let preds_vec = Vec::<f64>::try_from(&preds.get(0)).expect(
                &format!(
                    "Failed to convert tensor of dimensions {:?} to vector of type 'Vec<f64'!",
                    preds.size()
                )
                .to_string(),
            );
            // We're keeping only images that has a probability of being food higher than 50%
            preds_vec[0] < THRESHOLD
        })
        .map(|(_, asset)| asset.id.clone())
        .collect::<Vec<String>>();

    let push_response = push_assets_to_album(food_images_uuids, DESTINATION_ALBUM_ID).await?;

    println!("Added {} assets to album!", push_response.len());

    Ok(())
}
