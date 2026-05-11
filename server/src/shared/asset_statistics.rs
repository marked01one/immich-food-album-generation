use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetStatistics {
    pub videos: i64,
    pub images: i64,
    pub total: i64,
}
