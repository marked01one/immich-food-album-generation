mod shared;

use dotenvy::dotenv;

use shared::asset_statistics::AssetStatistics;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let immich_get_url = "http://hydrogen:2283/api/assets/statistics?apiKey";

    let api_key = std::env::var("IMMICH_API_KEY").expect("IMMICH_API_KEY must be set");

    let immich_get_url = format!("{immich_get_url}={api_key}");

    let resp = reqwest::get(immich_get_url)
        .await?
        .json::<AssetStatistics>()
        .await?;

    println!("{resp:#?}");

    Ok(())
}
