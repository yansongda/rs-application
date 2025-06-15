use crate::model::result::{Error, Result};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::Json;

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

impl Totp {
    pub fn ensure_permission(&self, user_id: i64) -> Result<()> {
        if self.user_id == user_id {
            return Ok(());
        }

        Err(Error::AuthorizationMiniprogramPermissionUngranted(None))
    }
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
