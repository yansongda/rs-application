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
    pub id: i64,
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

pub async fn fetch(user_id: i64) -> application_kernel::result::Result<User> {
    let sql = "select * from account.user where id = $1 limit 1";
    let started_at = Instant::now();

    let result: Option<User> = sqlx::query_as(sql)
        .bind(user_id)
        .fetch_optional(Pool::postgres("account")?)
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
) -> application_kernel::result::Result<User> {
    let sql = "insert into account.user (phone, config) values ($1, $2) returning *";
    let started_at = Instant::now();

    let result = sqlx::query_as(sql)
        .bind(phone)
        .bind(Json(&config))
        .fetch_one(Pool::postgres("account")?)
        .await
        .map_err(|e| {
            error!("插入用户失败: {:?}", e);

            Error::InternalDatabaseInsert(None)
        });

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, phone, ?config);

    result
}

pub async fn update_avatar(id: i64, avatar: &str) -> application_kernel::result::Result<User> {
    let sql = "update account.user set updated_at = now(), config = jsonb_set(config, '{avatar}', $1) where id = $2 returning *";
    let started_at = Instant::now();

    let result = sqlx::query_as(sql)
        .bind(Json(avatar))
        .bind(id)
        .fetch_one(Pool::postgres("account")?)
        .await
        .map_err(|e| {
            error!("更新用户失败: {:?}", e);

            Error::InternalDatabaseUpdate(None)
        });

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, id);

    result
}

pub async fn update_nickname(id: i64, nickname: &str) -> application_kernel::result::Result<User> {
    let sql = "update account.user set updated_at = now(), config = jsonb_set(config, '{nickname}', $1) where id = $2 returning *";
    let started_at = Instant::now();

    let result = sqlx::query_as(sql)
        .bind(Json(nickname))
        .bind(id)
        .fetch_one(Pool::postgres("account")?)
        .await
        .map_err(|e| {
            error!("更新昵称失败: {:?}", e);

            Error::InternalDatabaseUpdate(None)
        });

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, id);

    result
}

pub async fn update_slogan(id: i64, slogan: &str) -> application_kernel::result::Result<User> {
    let sql = "update entity.user set updated_at = now(), config = jsonb_set(config, '{slogan}', $1) where id = $2 returning *";
    let started_at = Instant::now();

    let result = sqlx::query_as(sql)
        .bind(Json(slogan))
        .bind(id)
        .fetch_one(Pool::postgres("entity")?)
        .await
        .map_err(|e| {
            error!("更新 Slogan 失败: {:?}", e);

            Error::InternalDatabaseUpdate(None)
        });

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, id);

    result
}

pub async fn update_phone(id: i64, phone: &str) -> application_kernel::result::Result<User> {
    let sql = "update entity.user set updated_at = now(), phone = $1 where id = $2 returning *";
    let started_at = Instant::now();

    let result = sqlx::query_as(sql)
        .bind(phone)
        .bind(id)
        .fetch_one(Pool::postgres("entity")?)
        .await
        .map_err(|e| {
            error!("更新手机号失败: {:?}", e);

            Error::InternalDatabaseUpdate(None)
        });

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, id);

    result
}
