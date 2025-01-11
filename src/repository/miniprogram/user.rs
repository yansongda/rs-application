use sqlx::{Execute, Postgres, QueryBuilder};
use std::time::Instant;
use tracing::{error, info};

use crate::model::miniprogram::user::{UpdateUser, User};
use crate::model::result::{Error, Result};
use crate::repository::Pool;

pub async fn fetch_by_open_id(open_id: &str) -> Result<User> {
    let sql = "select * from miniprogram.user where open_id = $1 limit 1";
    let started_at = Instant::now();

    let result: Option<User> = sqlx::query_as(sql)
        .bind(open_id)
        .fetch_optional(Pool::postgres("default"))
        .await
        .map_err(|e| {
            error!("查询用户失败: {:?}", e);

            Error::InternalDatabaseQuery(None)
        })?;

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, open_id);

    if let Some(user) = result {
        return Ok(user);
    }

    Err(Error::ParamsMiniprogramUserNotFound(None))
}

pub async fn insert(open_id: &str) -> Result<User> {
    let sql = "insert into miniprogram.user (open_id) values ($1) returning *";
    let started_at = Instant::now();

    let result = sqlx::query_as(sql)
        .bind(open_id)
        .fetch_one(Pool::postgres("default"))
        .await
        .map_err(|e| {
            error!("插入用户失败: {:?}", e);

            Error::InternalDatabaseInsert(None)
        });

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, open_id);

    result
}

pub async fn update(id: i64, update_user: UpdateUser) -> Result<User> {
    let mut builder =
        QueryBuilder::<Postgres>::new("update miniprogram.user set updated_at = now()");

    if let Some(ref nickname) = update_user.nickname {
        builder.push(", nickname = ").push_bind(nickname);
    }
    if let Some(ref avatar) = update_user.avatar {
        builder.push(", avatar = ").push_bind(avatar);
    }
    if let Some(ref slogan) = update_user.slogan {
        builder.push(", slogan = ").push_bind(slogan);
    }

    builder
        .push(" where id = ")
        .push_bind(id)
        .push(" returning *");

    let query = builder.build_query_as();
    let sql = query.sql();
    let started_at = Instant::now();

    let result = query
        .fetch_one(Pool::postgres("default"))
        .await
        .map_err(|e| {
            error!("更新用户失败: {:?}", e);

            Error::InternalDatabaseUpdate(None)
        });

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, id, ?update_user);

    result
}
