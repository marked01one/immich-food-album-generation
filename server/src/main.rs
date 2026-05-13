mod api;
mod utils;

use dotenvy::dotenv;

use api::responses::asset::AssetResponse;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let mut immich_get_url = "http://hydrogen:2283/api/assets/".to_string();

    let api_key = std::env::var("IMMICH_API_KEY").expect("IMMICH_API_KEY must be set");
    let image_uuid = "3c452695-7fd0-4b0a-9d50-d419c27150f0".to_string();

    immich_get_url.push_str(image_uuid.as_str());
    immich_get_url.push_str(format!("?apiKey={api_key}").as_str());

    let resp = reqwest::get(immich_get_url)
        .await?
        .json::<Option<AssetResponse>>()
        .await?
        .expect("Failed to get asset info!");

    println!("{resp:#?}");

    Ok(())
}
