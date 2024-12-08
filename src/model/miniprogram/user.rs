use crate::request::miniprogram::user::UpdateRequest;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i64,
    pub open_id: String,
    pub avatar: Option<String>,
    pub nickname: Option<String>,
    pub slogan: Option<String>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

#[derive(Debug, Clone)]
pub struct UpdateUser {
    pub avatar: Option<String>,
    pub nickname: Option<String>,
    pub slogan: Option<String>,
}

impl From<UpdateRequest> for UpdateUser {
    fn from(request: UpdateRequest) -> Self {
        Self {
            avatar: request.avatar,
            nickname: request.nickname,
            slogan: request.slogan,
        }
    }
}
