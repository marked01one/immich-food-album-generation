use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::utils::date_format;

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum UserAvatarColorEnum {
    primary,
    pink,
    red,
    yellow,
    blue,
    green,
    purple,
    orange,
    gray,
    amber,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct UserResponse {
    pub avatarColor: UserAvatarColorEnum,
    pub email: String,
    pub id: String,
    pub name: String,

    #[serde(with = "date_format")]
    pub profileChangedAt: DateTime<Utc>,
    pub profileImagePath: String,
}
