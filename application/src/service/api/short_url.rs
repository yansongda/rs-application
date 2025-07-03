use application_database::entity::tool::short_url::{CreateShortUrl, ShortUrl};
use application_kernel::result::Result;
use application_database::repository;
use fasthash::murmur3;

pub async fn create(url: &str) -> Result<ShortUrl> {
    let short = base62::encode(murmur3::hash32(url.as_bytes()));

    let result = repository::tool::short_url::fetch(&short).await;
    if result.is_ok() {
        return result;
    }

    repository::tool::short_url::insert(CreateShortUrl {
        url: url.to_string(),
        short,
    })
    .await
}

pub async fn detail(url: &str) -> Result<ShortUrl> {
    let result = repository::tool::short_url::fetch(url).await;

    if result.is_ok() {
        repository::tool::short_url::update_count(result.clone()?.id).await?;
    }

    result
}
