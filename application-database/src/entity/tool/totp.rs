use application_kernel::result::{Error, Result};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::Json;
use totp_rs::{Algorithm, Secret, TOTP};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Totp {
    pub id: i64,
    pub user_id: i64,
    pub username: String,
    pub issuer: Option<String>,
    pub config: Json<TotpConfig>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl Totp {
    pub fn ensure_permission(&self, user_id: i64) -> Result<()> {
        if self.user_id == user_id {
            return Ok(());
        }

        Err(Error::AuthorizationPermissionUngranted(None))
    }

    pub fn generate_code(&self) -> String {
        let config = self.config.clone();

        let totp = TOTP::new_unchecked(
            Algorithm::SHA1,
            6,
            1,
            config.period,
            Secret::Encoded(config.secret.clone()).to_bytes().unwrap(),
            self.issuer.clone(),
            self.username.clone(),
        );

        totp.generate_current().unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotpConfig {
    pub secret: String,
    pub period: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatedTotp {
    pub user_id: i64,
    pub username: String,
    pub issuer: Option<String>,
    pub config: TotpConfig,
}
