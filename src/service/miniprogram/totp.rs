use totp_rs::{Algorithm, Secret, TOTP};
use tracing::error;

use crate::model::miniprogram::totp::{CreateTotp, Totp, UpdateTotp};
use crate::model::result::{Error, Result};
use crate::repository::miniprogram;
use crate::request::miniprogram::totp::DetailResponse;

pub async fn all(user_id: i64) -> Result<Vec<DetailResponse>> {
    let totp = miniprogram::totp::all(user_id).await?;

    Ok(totp.into_iter().map(|t| t.into()).collect())
}

pub async fn detail(user_id: i64, id: i64) -> Result<DetailResponse> {
    let totp = miniprogram::totp::fetch(id).await?;

    if user_id != totp.user_id {
        return Err(Error::ParamsMiniprogramTotpNotFound(None));
    }

    Ok(totp.into())
}

pub async fn create(user_id: i64, uri: String) -> Result<()> {
    let totp = TOTP::from_url_unchecked(uri.as_str()).map_err(|e| {
        error!("TOTP 链接解析失败: {}", e);

        Error::ParamsMiniprogramTotpParseFailed(None)
    })?;

    miniprogram::totp::insert(CreateTotp {
        user_id,
        username: totp.account_name,
        issuer: totp.issuer,
        period: totp.step as i64,
        secret: Secret::Raw(totp.secret).to_encoded().to_string(),
    })
    .await?;

    Ok(())
}

pub async fn update(user_id: i64, params: UpdateTotp) -> Result<()> {
    let totp = miniprogram::totp::fetch(params.id).await?;

    if user_id != totp.user_id {
        return Err(Error::ParamsMiniprogramTotpNotFound(None));
    }

    miniprogram::totp::update(params).await?;

    Ok(())
}

pub async fn delete(user_id: i64, id: i64) -> Result<()> {
    let totp = miniprogram::totp::fetch(id).await?;

    if user_id != totp.user_id {
        return Err(Error::ParamsMiniprogramTotpNotFound(None));
    }

    miniprogram::totp::delete(id).await?;

    Ok(())
}

pub fn generate_code(totp: Totp) -> String {
    let config = totp.config.unwrap_or_default();

    let totp = TOTP::new_unchecked(
        Algorithm::SHA1,
        6,
        1,
        config.period as u64,
        Secret::Encoded(totp.secret).to_bytes().unwrap(),
        totp.issuer,
        totp.username,
    );

    totp.generate_current().unwrap()
}
