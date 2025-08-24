use crate::Pool;
use application_kernel::result::Error;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::time::Instant;
use tracing::{error, info};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ShortUrl {
    pub id: u64,
    pub short: String,
    pub url: String,
    pub visit: u64,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

#[derive(Debug)]
pub struct CreateShortUrl {
    pub url: String,
    pub short: String,
}

pub async fn fetch(short: &str) -> application_kernel::result::Result<ShortUrl> {
    let sql = "select * from tool.short_url where short = $1 limit 1";
    let started_at = Instant::now();

    let result: Option<ShortUrl> = sqlx::query_as(sql)
        .bind(short)
        .fetch_optional(Pool::mysql("tool")?)
        .await
        .map_err(|e| {
            error!("查询短连接失败: {:?}", e);

            Error::InternalDatabaseQuery(None)
        })?;

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, short);

    if let Some(short_url) = result {
        return Ok(short_url);
    }

    Err(Error::ParamsShortlinkNotFound(None))
}

pub async fn insert(url: CreateShortUrl) -> application_kernel::result::Result<ShortUrl> {
    let sql = "insert into tool.short_url (short, url) values ($1, $2) returning *";
    let started_at = Instant::now();

    let result = sqlx::query_as(sql)
        .bind(&url.short)
        .bind(&url.url)
        .fetch_one(Pool::mysql("tool")?)
        .await
        .map_err(|e| {
            error!("插入短连接失败: {:?}", e);

            Error::InternalDatabaseInsert(None)
        });

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, ?url);

    result
}

pub async fn update_count(id: u64) -> application_kernel::result::Result<bool> {
    let sql = "update tool.short_url set visit = visit + 1, updated_at = now() where id = $1";
    let started_at = Instant::now();

    sqlx::query(sql)
        .bind(id)
        .execute(Pool::mysql("tool")?)
        .await
        .map_err(|e| {
            error!("更新短连接访问次数失败: {:?}", e);

            Error::InternalDatabaseUpdate(None)
        })?;

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, id);

    Ok(true)
}
