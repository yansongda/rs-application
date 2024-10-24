use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ShortUrl {
    pub id: i64,
    pub short: String,
    pub url: String,
    pub visit: i64,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

#[derive(Debug)]
pub struct CreateShortUrl {
    pub url: String,
    pub short: String,
}
