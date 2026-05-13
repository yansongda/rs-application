use crate::{Pool, delete, insert, query_all, query_optional, update};
use application_kernel::result::{Error, Result};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::Acquire;
use sqlx::FromRow;
use sqlx::types::Json;
use std::time::Instant;
use totp_rs::{Algorithm, Secret, TOTP};
use tracing::{error, info};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortItem {
    pub id: u64,
    pub sort: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Totp {
    pub id: u64,
    pub user_id: u64,
    pub sort: u32,
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

    pub fn generate_code(&self) -> Result<String> {
        let config = &self.config;

        let secret = Secret::Encoded(config.secret.to_owned())
            .to_bytes()
            .map_err(|e| {
                error!("TOTP Secret 解码失败: {:?}", e);
                Error::ParamsTotpUriFormatInvalid(None)
            })?;

        let totp = TOTP::new_unchecked(
            Algorithm::SHA1,
            6,
            1,
            config.period,
            secret,
            self.issuer.to_owned(),
            self.username.to_owned(),
        );

        totp.generate_current().map_err(|e| {
            error!("TOTP Code 生成失败: {:?}", e);
            Error::InternalDatabaseQuery(None)
        })
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
    pub sort: Option<u32>,
    pub username: String,
    pub issuer: Option<String>,
    pub config: TotpConfig,
}

pub async fn all(user_id: u64) -> Result<Vec<Totp>> {
    let sql = "select * from tool.totp where user_id = ? order by sort desc, id asc";
    let pool = Pool::mysql("tool")?;

    Ok(query_all!(pool, sql, user_id))
}

pub async fn fetch(id: u64) -> Result<Totp> {
    let sql = "select * from tool.totp where id = ? limit 1";
    let pool = Pool::mysql("tool")?;

    let result: Option<Totp> = query_optional!(pool, sql, id);

    if let Some(user) = result {
        return Ok(user);
    }

    Err(Error::ParamsTotpNotFound(None))
}

pub async fn insert(totp: CreatedTotp) -> Result<Totp> {
    let sql = "insert into tool.totp (user_id, username, issuer, config) values (?, ?, ?, ?)";
    let pool = Pool::mysql("tool")?;

    let result = insert!(pool, sql, totp.user_id, &totp.username, &totp.issuer, Json(&(totp.config)));

    Ok(Totp {
        id: result.last_insert_id(),
        user_id: totp.user_id,
        sort: 0,
        username: totp.username,
        issuer: totp.issuer,
        config: Json(totp.config),
        created_at: Local::now(),
        updated_at: Local::now(),
    })
}

pub async fn update_issuer(id: u64, issuer: &str) -> Result<()> {
    let sql = "update tool.totp set issuer = ? where id = ?";
    let pool = Pool::mysql("tool")?;

    let _ = update!(pool, sql, issuer, id);

    Ok(())
}

pub async fn update_username(id: u64, username: &str) -> Result<()> {
    let sql = "update tool.totp set username = ? where id = ?";
    let pool = Pool::mysql("tool")?;

    let _ = update!(pool, sql, username, id);

    Ok(())
}

pub async fn delete(id: u64) -> Result<()> {
    let sql = "delete from tool.totp where id = ?";
    let pool = Pool::mysql("tool")?;

    let _ = delete!(pool, sql, id);

    Ok(())
}

pub async fn delete_by_user(user_id: u64) -> Result<()> {
    let sql = "delete from tool.totp where user_id = ?";
    let pool = Pool::mysql("tool")?;

    let _ = delete!(pool, sql, user_id);

    Ok(())
}

pub async fn sort(user_id: u64, items: &[SortItem]) -> Result<()> {
    let sql = "update tool.totp set sort = ? where id = ? and user_id = ?";
    let started_at = Instant::now();
    let pool = Pool::mysql("tool")?;
    let mut transaction = pool.begin().await.map_err(|e| {
        error!("开启 TOTP 排序事务失败: {:?}", e);

        Error::InternalDatabaseUpdate(None)
    })?;

    for item in items {
        let result = sqlx::query(sql)
            .bind(item.sort)
            .bind(item.id)
            .bind(user_id)
            .execute(transaction.acquire().await.map_err(|e| {
                error!("获取 TOTP 排序事务连接失败: {:?}", e);

                Error::InternalDatabaseUpdate(None)
            })?)
            .await
            .map_err(|e| {
                error!("批量更新 TOTP 排序失败: {:?}", e);

                Error::InternalDatabaseUpdate(None)
            })?;

        if result.rows_affected() != 1 {
            transaction.rollback().await.map_err(|e| {
                error!("回滚 TOTP 排序事务失败: {:?}", e);

                Error::InternalDatabaseUpdate(None)
            })?;

            return Err(Error::AuthorizationPermissionUngranted(None));
        }
    }

    transaction.commit().await.map_err(|e| {
        error!("提交 TOTP 排序事务失败: {:?}", e);

        Error::InternalDatabaseUpdate(None)
    })?;

    let elapsed = started_at.elapsed().as_secs_f32();
    info!(elapsed, sql, user_id, item_count = items.len());

    Ok(())
}
