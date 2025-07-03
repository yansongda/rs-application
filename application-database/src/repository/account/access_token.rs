use std::time::Instant;

use crate::entity::account::access_token::{AccessToken, AccessTokenData};
use crate::entity::account::third_user::Platform;
use application_kernel::result::{Error, Result};
use crate::repository::Pool;
use sqlx::types::Json;
use tracing::{error, info};

pub async fn fetch(access_token: &str) -> Result<AccessToken> {
    let sql = "select * from account.access_token where access_token = $1 limit 1";

    let result: Option<AccessToken> = sqlx::query_as(sql)
        .bind(access_token)
        .fetch_optional(Pool::postgres("account")?)
        .await
        .map_err(|e| {
            error!("查询 access_token 失败: {:?}", e);

            Error::InternalDatabaseQuery(None)
        })?;

    if let Some(data) = result {
        return Ok(data);
    }

    Err(Error::ParamsAccessTokenNotFound(None))
}

pub async fn fetch_by_user_id(user_id: i64) -> Result<AccessToken> {
    let sql = "select * from account.access_token where user_id = $1 limit 1";
    let started_at = Instant::now();

    let result: Option<AccessToken> = sqlx::query_as(sql)
        .bind(user_id)
        .fetch_optional(Pool::postgres("account")?)
        .await
        .map_err(|e| {
            error!("通过 user_id 查询 access_token 失败: {:?}", e);

            Error::InternalDatabaseQuery(None)
        })?;

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, user_id);

    if let Some(data) = result {
        return Ok(data);
    }

    Err(Error::ParamsAccessTokenNotFound(None))
}

pub async fn insert(
    platform: Platform,
    user_id: i64,
    data: &AccessTokenData,
) -> Result<AccessToken> {
    let sql = "insert into account.access_token (user_id, access_token, data, platform) values ($1, $2, $3, $4) returning *";
    let access_token = data.to_access_token(&platform)?;
    let started_at = Instant::now();

    let result = sqlx::query_as(sql)
        .bind(user_id)
        .bind(&access_token)
        .bind(Json(data))
        .bind(platform)
        .fetch_one(Pool::postgres("account")?)
        .await
        .map_err(|e| {
            error!("插入 access_token 失败: {:?}", e);

            Error::InternalDatabaseInsert(None)
        });

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, user_id, access_token, ?data);

    result
}

pub async fn update(access_token: AccessToken, data: &AccessTokenData) -> Result<AccessToken> {
    let sql = "update account.access_token set access_token = $1, data = $2, updated_at = now() where id = $3 returning *";
    let started_at = Instant::now();

    let result = sqlx::query_as(sql)
        .bind(data.to_access_token(&access_token.platform)?)
        .bind(Json(data))
        .bind(access_token.id)
        .fetch_one(Pool::postgres("account")?)
        .await
        .map_err(|e| {
            error!("更新 access_token 失败: {:?}", e);

            Error::InternalDatabaseUpdate(None)
        });

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, access_token.id, ?data);

    result
}
