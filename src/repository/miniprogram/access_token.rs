use fasthash::murmur3;
use std::time::Instant;

use sqlx::types::Json;
use tracing::{error, info};

use crate::model::miniprogram::wechat_access_token::{AccessToken, AccessTokenData};
use crate::model::result::{Error, Result};
use crate::repository::Pool;

pub async fn fetch(access_token: &str) -> Result<AccessToken> {
    let sql = "select * from miniprogram.access_token where access_token = $1 limit 1";
    let started_at = Instant::now();

    let result: Option<AccessToken> = sqlx::query_as(sql)
        .bind(access_token)
        .fetch_optional(Pool::postgres("miniprogram")?)
        .await
        .map_err(|e| {
            error!("查询 access_token 失败: {:?}", e);

            Error::InternalDatabaseQuery(None)
        })?;

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, access_token);

    if let Some(data) = result {
        return Ok(data);
    }

    Err(Error::ParamsMiniprogramAccessTokenNotFound(None))
}

pub async fn fetch_by_user_id(user_id: i64) -> Result<AccessToken> {
    let sql = "select * from miniprogram.access_token where user_id = $1 limit 1";
    let started_at = Instant::now();

    let result: Option<AccessToken> = sqlx::query_as(sql)
        .bind(user_id)
        .fetch_optional(Pool::postgres("miniprogram")?)
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

    Err(Error::ParamsMiniprogramAccessTokenNotFound(None))
}

pub async fn insert(user_id: i64, data: AccessTokenData) -> Result<AccessToken> {
    let sql = "insert into miniprogram.access_token (user_id, access_token, data) values ($1, $2, $3) returning *";
    let access_token = base62::encode(murmur3::hash128(
        format!("{}:{}", &data.open_id, &data.session_key).as_bytes(),
    ));
    let started_at = Instant::now();

    let result = sqlx::query_as(sql)
        .bind(user_id)
        .bind(&access_token)
        .bind(Json(&data))
        .fetch_one(Pool::postgres("miniprogram")?)
        .await
        .map_err(|e| {
            error!("插入 access_token 失败: {:?}", e);

            Error::InternalDatabaseInsert(None)
        });

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, user_id, access_token, ?data);

    result
}

pub async fn update(id: i64, data: AccessTokenData) -> Result<AccessToken> {
    let sql = "update miniprogram.access_token set access_token = $1, data = $2, updated_at = now() where id = $3 returning *";
    let access_token = base62::encode(murmur3::hash128(
        format!("{}:{}", &data.open_id, &data.session_key).as_bytes(),
    ));
    let started_at = Instant::now();

    let result = sqlx::query_as(sql)
        .bind(access_token)
        .bind(Json(&data))
        .bind(id)
        .fetch_one(Pool::postgres("miniprogram")?)
        .await
        .map_err(|e| {
            error!("更新 access_token 失败: {:?}", e);

            Error::InternalDatabaseUpdate(None)
        });

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, id, ?data);

    result
}
