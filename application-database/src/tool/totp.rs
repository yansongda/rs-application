use crate::Pool;
use application_kernel::result::{Error, Result};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::Json;
use std::time::Instant;
use totp_rs::{Algorithm, Secret, TOTP};
use tracing::{error, info};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Totp {
    pub id: u64,
    pub user_id: u64,
    pub username: String,
    pub issuer: Option<String>,
    pub config: Json<TotpConfig>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl Totp {
    pub fn ensure_permission(&self, user_id: u64) -> Result<()> {
        if self.user_id == user_id {
            return Ok(());
        }

        Err(Error::AuthorizationPermissionUngranted(None))
    }

    pub fn generate_code(&self) -> String {
        let config = &self.config;

        let totp = TOTP::new_unchecked(
            Algorithm::SHA1,
            6,
            1,
            config.period,
            Secret::Encoded(config.secret.to_owned()).to_bytes().unwrap(),
            self.issuer.to_owned(),
            self.username.to_owned(),
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
    pub user_id: u64,
    pub username: String,
    pub issuer: Option<String>,
    pub config: TotpConfig,
}

pub async fn all(user_id: u64) -> Result<Vec<Totp>> {
    let sql = "select * from tool.totp where user_id = ? order by id asc";
    let started_at = Instant::now();

    let result = sqlx::query_as(sql)
        .bind(user_id)
        .fetch_all(Pool::mysql("tool")?)
        .await
        .map_err(|e| {
            error!("查询用户所有的 Totp 失败: {:?}", e);

            Error::InternalDatabaseQuery(None)
        });

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, user_id);

    result
}

pub async fn fetch(id: u64) -> Result<Totp> {
    let sql = "select * from tool.totp where id = ? limit 1";
    let started_at = Instant::now();

    let result: Option<Totp> = sqlx::query_as(sql)
        .bind(id)
        .fetch_optional(Pool::mysql("tool")?)
        .await
        .map_err(|e| {
            error!("查询 Totp 详情失败: {:?}", e);

            Error::InternalDatabaseQuery(None)
        })?;

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, id);

    if let Some(user) = result {
        return Ok(user);
    }

    Err(Error::ParamsTotpNotFound(None))
}

pub async fn insert(totp: CreatedTotp) -> Result<u64> {
    let sql = "insert into tool.totp (user_id, username, issuer, config) values (?, ?, ?, ?)";
    let started_at = Instant::now();

    let result = sqlx::query(sql)
        .bind(totp.user_id)
        .bind(&totp.username)
        .bind(&totp.issuer)
        .bind(Json(&(totp.config)))
        .execute(Pool::mysql("tool")?)
        .await
        .map_err(|e| {
            error!("插入 Totp 失败: {:?}", e);

            Error::InternalDatabaseInsert(None)
        });

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, ?totp);

    Ok(result?.last_insert_id())
}

pub async fn update_issuer(id: u64, issuer: &str) -> Result<()> {
    let sql = "update tool.totp set issuer = ? where id = ?";
    let started_at = Instant::now();

    let _ = sqlx::query(sql)
        .bind(issuer)
        .bind(id)
        .execute(Pool::mysql("tool")?)
        .await
        .map_err(|e| {
            error!("更新 TOTP 的 issuer 失败: {:?}", e);

            Error::InternalDatabaseUpdate(None)
        });

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, id);

    Ok(())
}

pub async fn update_username(id: u64, username: &str) -> Result<()> {
    let sql = "update tool.totp set username = ? where id = ?";
    let started_at = Instant::now();

    let _ = sqlx::query(sql)
        .bind(username)
        .bind(id)
        .execute(Pool::mysql("tool")?)
        .await
        .map_err(|e| {
            error!("更新 TOTP 的 username 失败: {:?}", e);

            Error::InternalDatabaseUpdate(None)
        });

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, id);

    Ok(())
}

pub async fn delete(id: u64) -> Result<()> {
    let sql = "delete from tool.totp where id = $1";
    let started_at = Instant::now();

    sqlx::query(sql)
        .bind(id)
        .execute(Pool::mysql("tool")?)
        .await
        .map_err(|e| {
            error!("删除 Totp 失败: {:?}", e);

            Error::InternalDatabaseDelete(None)
        })?;

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, id);

    Ok(())
}
