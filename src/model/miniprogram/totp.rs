use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::Json;

use crate::request::miniprogram::totp::{EditIssuerRequest, EditUsernameRequest, UpdateRequest};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Totp {
    pub id: i64,
    pub user_id: i64,
    pub username: String,
    pub issuer: Option<String>,
    pub secret: String,
    pub config: Option<Json<TotpConfig>>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotpConfig {
    pub period: i64,
}

impl Default for TotpConfig {
    fn default() -> Self {
        Self { period: 30 }
    }
}

#[derive(Debug, Clone)]
pub struct CreatedTotp {
    pub user_id: i64,
    pub username: String,
    pub issuer: Option<String>,
    pub period: i64,
    pub secret: String,
}

#[derive(Debug, Clone)]
pub struct UpdatedTotp {
    pub id: i64,
    pub issuer: Option<String>,
    pub username: Option<String>,
}

impl From<EditIssuerRequest> for UpdatedTotp {
    fn from(request: EditIssuerRequest) -> Self {
        Self {
            id: request.id.unwrap(),
            username: None,
            issuer: request.issuer,
        }
    }
}

impl From<EditUsernameRequest> for UpdatedTotp {
    fn from(request: EditUsernameRequest) -> Self {
        Self {
            id: request.id.unwrap(),
            username: request.username,
            issuer: None,
        }
    }
}

impl From<UpdateRequest> for UpdatedTotp {
    fn from(request: UpdateRequest) -> Self {
        Self {
            id: request.id.unwrap(),
            username: request.username,
            issuer: request.issuer,
        }
    }
}
