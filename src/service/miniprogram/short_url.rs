use crate::model::miniprogram::short_url::{CreateShortUrl, ShortUrl};
use crate::model::result::Result;
use crate::repository::miniprogram;
use fasthash::murmur3;

pub async fn create(url: &str) -> Result<ShortUrl> {
    let short = base62::encode(murmur3::hash32(url.as_bytes()));

    let result = miniprogram::short_url::fetch(&short).await;
    if result.is_ok() {
        return result;
    }

    miniprogram::short_url::insert(CreateShortUrl {
        url: url.to_string(),
        short,
    })
    .await
}

pub async fn detail(url: &str) -> Result<ShortUrl> {
    let result = miniprogram::short_url::fetch(url).await;

    if result.is_ok() {
        miniprogram::short_url::update_count(result.clone()?.id).await?;
    }

    result
}
