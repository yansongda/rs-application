use crate::Pool;
use application_kernel::result::Error;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::Json;
use std::time::Instant;
use tracing::{error, info};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: u64,
    pub phone: Option<String>,
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

pub async fn fetch(user_id: u64) -> application_kernel::result::Result<User> {
    let sql = "select * from account.user where id = ? limit 1";
    let started_at = Instant::now();

    let result: Option<User> = sqlx::query_as(sql)
        .bind(user_id)
        .fetch_optional(Pool::mysql("account")?)
        .await
        .map_err(|e| {
            error!("查询用户失败: {:?}", e);

            Error::InternalDatabaseQuery(None)
        })?;

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, user_id);

    if let Some(user) = result {
        return Ok(user);
    }

    Err(Error::ParamsUserNotFound(None))
}

pub async fn insert(
    phone: Option<&str>,
    config: Config,
) -> application_kernel::result::Result<u64> {
    let sql = "insert into account.user (phone, config) values (?, ?)";
    let started_at = Instant::now();

    let result = sqlx::query(sql)
        .bind(phone)
        .bind(Json(&config))
        .execute(Pool::mysql("account")?)
        .await
        .map_err(|e| {
            error!("插入用户失败: {:?}", e);

            Error::InternalDatabaseInsert(None)
        });

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, phone, ?config);

    Ok(result?.last_insert_id())
}

pub async fn update_avatar(id: u64, avatar: &str) -> application_kernel::result::Result<()> {
    let sql = "update account.user set config = json_set(config, '$.avatar', ?) where id = ?";
    let started_at = Instant::now();

    let _ = sqlx::query(sql)
        .bind(avatar)
        .bind(id)
        .execute(Pool::mysql("account")?)
        .await
        .map_err(|e| {
            error!("更新用户失败: {:?}", e);

            Error::InternalDatabaseUpdate(None)
        });

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, id);

    Ok(())
}

pub async fn update_nickname(id: u64, nickname: &str) -> application_kernel::result::Result<()> {
    let sql = "update account.user set config = json_set(config, '$.nickname', ?) where id = ?";
    let started_at = Instant::now();

    let _ = sqlx::query(sql)
        .bind(nickname)
        .bind(id)
        .execute(Pool::mysql("account")?)
        .await
        .map_err(|e| {
            error!("更新昵称失败: {:?}", e);

            Error::InternalDatabaseUpdate(None)
        });

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, id);

    Ok(())
}

pub async fn update_slogan(id: u64, slogan: &str) -> application_kernel::result::Result<()> {
    let sql = "update account.user set config = json_set(config, '$.slogan', ?) where id = ?";
    let started_at = Instant::now();

    let _ = sqlx::query(sql)
        .bind(slogan)
        .bind(id)
        .fetch_one(Pool::mysql("account")?)
        .await
        .map_err(|e| {
            error!("更新 Slogan 失败: {:?}", e);

            Error::InternalDatabaseUpdate(None)
        });

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, id);

    Ok(())
}

pub async fn update_phone(id: u64, phone: &str) -> application_kernel::result::Result<()> {
    let sql = "update account.user set phone = ? where id = ?";
    let started_at = Instant::now();

    let _ = sqlx::query(sql)
        .bind(phone)
        .bind(id)
        .execute(Pool::mysql("account")?)
        .await
        .map_err(|e| {
            error!("更新手机号失败: {:?}", e);

            Error::InternalDatabaseUpdate(None)
        });

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, id);

    Ok(())
}
