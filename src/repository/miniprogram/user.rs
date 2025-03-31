use sqlx::types::Json;
use sqlx::{Execute, Postgres, QueryBuilder};
use std::time::Instant;
use tracing::{error, info};

use crate::model::miniprogram::user::{EditUser, User};
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

pub async fn insert(data: EditUser) -> Result<User> {
    let sql = "insert into miniprogram.user (phone, config) values ($1, $2) returning *";
    let started_at = Instant::now();

    let result = sqlx::query_as(sql)
        .bind(data.clone().phone.unwrap_or("".to_string()))
        .bind(Json(data.clone().config))
        .fetch_one(Pool::postgres("miniprogram")?)
        .await
        .map_err(|e| {
            error!("插入用户失败: {:?}", e);

            Error::InternalDatabaseInsert(None)
        });

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, ?data);

    result
}

pub async fn update(id: i64, edit_user: EditUser) -> Result<User> {
    let mut builder =
        QueryBuilder::<Postgres>::new("update miniprogram.user set updated_at = now()");

    if let Some(ref phone) = edit_user.phone {
        builder.push(", phone = ").push_bind(phone);
    }
    if let Some(ref config) = edit_user.config {
        builder.push(", config = ").push_bind(Json(config));
    }

    builder
        .push(" where id = ")
        .push_bind(id)
        .push(" returning *");

    let query = builder.build_query_as();
    let sql = query.sql();
    let started_at = Instant::now();

    let result = query
        .fetch_one(Pool::postgres("miniprogram")?)
        .await
        .map_err(|e| {
            error!("更新用户失败: {:?}", e);

            Error::InternalDatabaseUpdate(None)
        });

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, id, ?edit_user);

    result
}
