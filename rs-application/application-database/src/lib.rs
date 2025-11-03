use application_kernel::config::{Database, G_CONFIG};
use application_kernel::result::{Error, Result};
use sqlx::MySqlPool;
use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::LazyLock;
use std::time::Duration;
use tracing::error;

pub mod account;
pub mod tool;

pub struct Pool;

static G_POOL_MYSQL: LazyLock<HashMap<&'static str, MySqlPool>> = LazyLock::new(|| {
    let databases = &G_CONFIG.databases;

    let mut mysql: HashMap<&'static str, MySqlPool> = HashMap::new();

    for database in databases {
        if database.1.url.starts_with("mysql://") {
            mysql.insert(database.0, Pool::connect_mysql(database.1));
        }
    }

    mysql
});

impl Pool {
    pub fn mysql(pool: &str) -> Result<&MySqlPool> {
        G_POOL_MYSQL.get(pool).ok_or_else(|| {
            error!("获取数据库连接失败: {}", pool);

            Error::InternalDatabaseAcquire(None)
        })
    }

    fn connect_mysql(config: &Database) -> MySqlPool {
        let connection_options = MySqlConnectOptions::from_str(config.url.as_str()).unwrap();

        MySqlPoolOptions::new()
            .acquire_timeout(Duration::from_secs(config.acquire_timeout))
            .min_connections(config.min_connections)
            .max_connections(config.max_connections)
            .idle_timeout(Duration::from_secs(config.idle_timeout))
            .connect_lazy_with(connection_options)
    }
}
