use crate::model::wechat::LoginResponse;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AccessToken {
    pub id: i64,
    pub user_id: i64,
    pub access_token: String,
    pub data: Json<AccessTokenData>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessTokenData {
    pub open_id: String,
    pub session_key: String,
    pub union_id: Option<String>,
}

impl From<LoginResponse> for AccessTokenData {
    fn from(response: LoginResponse) -> Self {
        AccessTokenData {
            open_id: response.openid.unwrap(),
            session_key: response.session_key.unwrap(),
            union_id: response.unionid,
        }
    }
}
