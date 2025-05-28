use crate::request::miniprogram::user::{EditNicknameRequest, EditRequest};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::Json;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i64,
    pub phone: String,
    pub config: Option<Json<Config>>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub avatar: Option<String>,
    pub nickname: Option<String>,
    pub slogan: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditUser {
    pub phone: Option<String>,
    pub config: Option<Config>,
}

impl From<EditNicknameRequest> for EditUser {
    fn from(request: EditNicknameRequest) -> Self {
        Self {
            phone: None,
            config: Some(Config {
                avatar: None,
                nickname: request.nickname,
                slogan: None,
            }),
        }
    }
}

impl From<EditRequest> for EditUser {
    fn from(request: EditRequest) -> Self {
        Self {
            phone: request.phone,
            config: request.config,
        }
    }
}
