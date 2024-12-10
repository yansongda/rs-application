use config::{Config as C, Environment, File};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::LazyLock;

pub(crate) static G_CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let config = C::builder()
        .add_source(File::with_name("./config.toml").required(false))
        .add_source(
            Environment::with_prefix("APP")
                .try_parsing(true)
                .separator("_"),
        )
        .build()
        .expect("加载配置失败");

    let instance = config.try_deserialize::<Config>().expect("解析配置失败");

    instance
});

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub name: String,
    pub bin: HashMap<String, Bin>,
    pub databases: HashMap<String, Database>,
    pub wechat: Wechat,
    pub short_url: ShortUrl,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Bin {
    pub listen: String,
    pub port: u16,
    pub debug: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Database {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Wechat {
    pub url: String,
    pub app_id: String,
    pub app_secret: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ShortUrl {
    pub domain: String,
}
