use crate::Pool;
use application_kernel::config::G_CONFIG;
use application_kernel::result::{Error, Result};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::time::Instant;
use tracing::{error, info};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RefreshToken {
    pub id: u64,
    pub access_token_id: u64,
    pub refresh_token: String,
    pub expired_at: DateTime<Local>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl RefreshToken {
    pub fn is_expired(&self) -> bool {
        Local::now() < self.expired_at
    }
}

pub async fn fetch(refresh_token: &str) -> Result<RefreshToken> {
    let sql = "select * from account.refresh_token where refresh_token = ? limit 1";
    let started_at = Instant::now();

    let result: Option<RefreshToken> = sqlx::query_as(sql)
        .bind(refresh_token)
        .fetch_optional(Pool::mysql("account")?)
        .await
        .map_err(|e| {
            error!("查询 refresh_token 失败: {:?}", e);

            Error::InternalDatabaseQuery(None)
        })?;

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, refresh_token);

    if let Some(data) = result {
        return Ok(data);
    }

    Err(Error::ParamsAccessTokenNotFound(None))
}

pub async fn insert(access_token_id: u64) -> Result<RefreshToken> {
    let sql = "insert into account.refresh_token (access_token_id, refresh_token, expired_at) values (?, ?, ?)";
    let refresh_token = Uuid::now_v7().to_string();
    let expired_at =
        Local::now() + chrono::Duration::seconds(G_CONFIG.access_token.refresh_expired_in as i64);
    let started_at = Instant::now();

    let result = sqlx::query(sql)
        .bind(access_token_id)
        .bind(&refresh_token)
        .bind(&expired_at)
        .execute(Pool::mysql("account")?)
        .await
        .map_err(|e| {
            error!("插入 refresh_token 失败: {:?}", e);

            Error::InternalDatabaseInsert(None)
        });

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, access_token_id);

    Ok(RefreshToken {
        id: result?.last_insert_id(),
        access_token_id,
        refresh_token,
        expired_at,
        created_at: Local::now(),
        updated_at: Local::now(),
    })
}
