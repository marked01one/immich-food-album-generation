use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AddAssetsToAlbumRequest {
    pub ids: Vec<String>,
}

impl AddAssetsToAlbumRequest {
    pub fn new(ids: Vec<String>) -> Self {
        Self { ids }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct CreateAlbumRequest {
    pub albumName: String,
}
