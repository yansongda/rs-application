// TODO: 上面所有的 SQL 查询，应该都可以封装为一个属性宏来实现，公共处理 sql，查询时间 等信息，这样可以减少代码量，提高可读性

pub mod access_token;
pub mod short_url;
pub mod third_user;
pub mod totp;
pub mod user;
