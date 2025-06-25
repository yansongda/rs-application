use std::collections::HashMap;
use std::str::FromStr;
use std::sync::LazyLock;
use std::time::Duration;

use crate::config::{Database, G_CONFIG};
use crate::model::result::{Error, Result};
use sqlx::PgPool;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use tracing::error;

pub mod access_token;
pub mod short_url;
pub mod third_user;
pub mod totp;
pub mod user;

// TODO: 上面所有的 SQL 查询，应该都可以封装为一个属性宏来实现，公共处理 sql，查询时间 等信息，这样可以减少代码量，提高可读性

pub struct Pool;

static G_POOL_PG: LazyLock<HashMap<&'static str, PgPool>> = LazyLock::new(|| {
    let databases = &G_CONFIG.databases;

    let mut pg: HashMap<&'static str, PgPool> = HashMap::new();

    for database in databases {
        if database.1.url.starts_with("postgres://") {
            pg.insert(database.0, Pool::connect_postgres(database.1));
        }
    }

    pg
});

impl Pool {
    pub fn postgres(pool: &str) -> Result<&PgPool> {
        G_POOL_PG.get(pool).ok_or_else(|| {
            error!("获取数据库连接失败: {}", pool);

            Error::InternalDatabaseAcquire(None)
        })
    }

    fn connect_postgres(config: &Database) -> PgPool {
        let connection_options = PgConnectOptions::from_str(config.url.as_str())
            .unwrap()
            .application_name(G_CONFIG.name.as_str());

        PgPoolOptions::new()
            .acquire_timeout(Duration::from_secs(config.acquire_timeout))
            .min_connections(config.min_connections)
            .max_connections(config.max_connections)
            .idle_timeout(Duration::from_secs(config.idle_timeout))
            .connect_lazy_with(connection_options)
    }
}
