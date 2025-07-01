use application_database::::short_url::{CreateShortUrl, ShortUrl};
use application_kernel::result::Result;
use crate::repository;
use fasthash::murmur3;

pub async fn create(url: &str) -> Result<ShortUrl> {
    let short = base62::encode(murmur3::hash32(url.as_bytes()));

    let result = repository::short_url::fetch(&short).await;
    if result.is_ok() {
        return result;
    }

    repository::short_url::insert(CreateShortUrl {
        url: url.to_string(),
        short,
    })
    .await
}

pub async fn detail(url: &str) -> Result<ShortUrl> {
    let result = repository::short_url::fetch(url).await;

    if result.is_ok() {
        repository::short_url::update_count(result.clone()?.id).await?;
    }

    result
}
