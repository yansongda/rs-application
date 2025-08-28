use application_database::tool::short_url;
use application_kernel::result::Result;
use fasthash::murmur3;

pub async fn create(url: &str) -> Result<short_url::ShortUrl> {
    let short = base62::encode(murmur3::hash32(url.as_bytes()));

    let result = short_url::fetch(&short).await;
    if result.is_ok() {
        return result;
    }

    short_url::insert(short_url::CreateShortUrl {
        url: url.to_string(),
        short,
    })
    .await
}

pub async fn detail(url: &str) -> Result<short_url::ShortUrl> {
    let result = short_url::fetch(url).await;

    if let Ok(short_url) = &result {
        short_url::update_count(short_url.id).await?;
    }

    result
}
