use sqlx::types::Json;
use std::time::Instant;
use tracing::{error, info};

use crate::model::miniprogram::user::{Config, User};
use crate::model::result::{Error, Result};
use crate::repository::Pool;

pub async fn fetch(user_id: i64) -> Result<User> {
    let sql = "select * from miniprogram.user where id = $1 limit 1";
    let started_at = Instant::now();

    let result: Option<User> = sqlx::query_as(sql)
        .bind(user_id)
        .fetch_optional(Pool::postgres("miniprogram")?)
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

    Err(Error::ParamsMiniprogramUserNotFound(None))
}

pub async fn insert(phone: Option<&str>, config: Config) -> Result<User> {
    let sql = "insert into miniprogram.user (phone, config) values ($1, $2) returning *";
    let started_at = Instant::now();

    let result = sqlx::query_as(sql)
        .bind(phone)
        .bind(Json(&config))
        .fetch_one(Pool::postgres("miniprogram")?)
        .await
        .map_err(|e| {
            error!("插入用户失败: {:?}", e);

            Error::InternalDatabaseInsert(None)
        });

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, phone, ?config);

    result
}

pub async fn update_avatar(id: i64, avatar: &str) -> Result<User> {
    let sql = "update miniprogram.user set updated_at = now(), config = jsonb_set(config, '{avatar}', $1) where id = $2 returning *";
    let started_at = Instant::now();

    let result = sqlx::query_as(sql)
        .bind(Json(avatar))
        .bind(id)
        .fetch_one(Pool::postgres("miniprogram")?)
        .await
        .map_err(|e| {
            error!("更新用户失败: {:?}", e);

            Error::InternalDatabaseUpdate(None)
        });

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, id);

    result
}

pub async fn update_nickname(id: i64, nickname: &str) -> Result<User> {
    let sql = "update miniprogram.user set updated_at = now(), config = jsonb_set(config, '{nickname}', $1) where id = $2 returning *";
    let started_at = Instant::now();

    let result = sqlx::query_as(sql)
        .bind(Json(nickname))
        .bind(id)
        .fetch_one(Pool::postgres("miniprogram")?)
        .await
        .map_err(|e| {
            error!("更新昵称失败: {:?}", e);

            Error::InternalDatabaseUpdate(None)
        });

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, id);

    result
}

pub async fn update_slogan(id: i64, slogan: &str) -> Result<User> {
    let sql = "update miniprogram.user set updated_at = now(), config = jsonb_set(config, '{slogan}', $1) where id = $2 returning *";
    let started_at = Instant::now();

    let result = sqlx::query_as(sql)
        .bind(Json(slogan))
        .bind(id)
        .fetch_one(Pool::postgres("miniprogram")?)
        .await
        .map_err(|e| {
            error!("更新 Slogan 失败: {:?}", e);

            Error::InternalDatabaseUpdate(None)
        });

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, id);

    result
}

pub async fn update_phone(id: i64, phone: &str) -> Result<User> {
    let sql =
        "update miniprogram.user set updated_at = now(), phone = $1 where id = $2 returning *";
    let started_at = Instant::now();

    let result = sqlx::query_as(sql)
        .bind(phone)
        .bind(id)
        .fetch_one(Pool::postgres("miniprogram")?)
        .await
        .map_err(|e| {
            error!("更新手机号失败: {:?}", e);

            Error::InternalDatabaseUpdate(None)
        });

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, id);

    result
}
