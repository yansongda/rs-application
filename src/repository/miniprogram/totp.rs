use sqlx::types::Json;
use std::time::Instant;
use tracing::{error, info};

use crate::model::miniprogram::totp::{CreatedTotp, Totp, TotpConfig};
use crate::model::miniprogram::user::User;
use crate::model::result::{Error, Result};
use crate::repository::Pool;

pub async fn all(user_id: i64) -> Result<Vec<Totp>> {
    let sql = "select * from miniprogram.totp where user_id = $1";
    let started_at = Instant::now();

    let result = sqlx::query_as(sql)
        .bind(user_id)
        .fetch_all(Pool::postgres("miniprogram")?)
        .await
        .map_err(|e| {
            error!("查询用户所有的 Totp 失败: {:?}", e);

            Error::InternalDatabaseQuery(None)
        });

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, user_id);

    result
}

pub async fn fetch(id: i64) -> Result<Totp> {
    let sql = "select * from miniprogram.totp where id = $1 limit 1";
    let started_at = Instant::now();

    let result: Option<Totp> = sqlx::query_as(sql)
        .bind(id)
        .fetch_optional(Pool::postgres("miniprogram")?)
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

    Err(Error::ParamsMiniprogramTotpNotFound(None))
}

pub async fn insert(totp: CreatedTotp) -> Result<Totp> {
    let sql = "insert into miniprogram.totp (user_id, username, issuer, secret, config) values ($1, $2, $3, $4, $5) returning *";
    let started_at = Instant::now();

    let result = sqlx::query_as(sql)
        .bind(totp.user_id)
        .bind(&totp.username)
        .bind(&totp.issuer)
        .bind(&totp.secret)
        .bind(Json(TotpConfig {
            period: totp.period,
        }))
        .fetch_one(Pool::postgres("miniprogram")?)
        .await
        .map_err(|e| {
            error!("插入 Totp 失败: {:?}", e);

            Error::InternalDatabaseInsert(None)
        });

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, ?totp);

    result
}

pub async fn update_issuer(id: i64, issuer: &str) -> Result<User> {
    let sql =
        "update miniprogram.totp set updated_at = now(), issuer = $1 where id = $2 returning *";
    let started_at = Instant::now();

    let result = sqlx::query_as(sql)
        .bind(issuer)
        .bind(id)
        .fetch_one(Pool::postgres("miniprogram")?)
        .await
        .map_err(|e| {
            error!("更新 TOTP 的 issuer 失败: {:?}", e);

            Error::InternalDatabaseUpdate(None)
        });

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, id);

    result
}

pub async fn update_username(id: i64, username: &str) -> Result<User> {
    let sql =
        "update miniprogram.totp set updated_at = now(), username = $1 where id = $2 returning *";
    let started_at = Instant::now();

    let result = sqlx::query_as(sql)
        .bind(username)
        .bind(id)
        .fetch_one(Pool::postgres("miniprogram")?)
        .await
        .map_err(|e| {
            error!("更新 TOTP 的 username 失败: {:?}", e);

            Error::InternalDatabaseUpdate(None)
        });

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, id);

    result
}

pub async fn delete(id: i64) -> Result<()> {
    let sql = "delete from miniprogram.totp where id = $1";
    let started_at = Instant::now();

    sqlx::query(sql)
        .bind(id)
        .execute(Pool::postgres("miniprogram")?)
        .await
        .map_err(|e| {
            error!("删除 Totp 失败: {:?}", e);

            Error::InternalDatabaseDelete(None)
        })?;

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, id);

    Ok(())
}
