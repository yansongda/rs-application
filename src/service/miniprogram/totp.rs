use crate::model::miniprogram::access_token::AccessToken;
use crate::model::miniprogram::totp::{CreatedTotp, Totp};
use crate::model::result::{Error, Result};
use crate::repository::miniprogram;
use crate::request::miniprogram::totp::{
    DetailResponse, EditIssuerRequestParams, EditUsernameRequestParams,
};
use totp_rs::{Algorithm, Secret, TOTP};
use tracing::error;

pub async fn all(access_token: AccessToken) -> Result<Vec<DetailResponse>> {
    let totp = miniprogram::totp::all(access_token.user_id).await?;

    Ok(totp.into_iter().map(|t| t.into()).collect())
}

pub async fn detail(access_token: AccessToken, id: i64) -> Result<DetailResponse> {
    let totp = miniprogram::totp::fetch(id).await?;

    totp.ensure_permission(access_token.user_id)?;

    Ok(totp.into())
}

pub async fn create(access_token: AccessToken, uri: String) -> Result<()> {
    let totp = TOTP::from_url_unchecked(uri.as_str()).map_err(|e| {
        error!("TOTP 链接解析失败: {}", e);

        Error::ParamsMiniprogramTotpParseFailed(None)
    })?;

    miniprogram::totp::insert(CreatedTotp {
        user_id: access_token.user_id,
        username: totp.account_name,
        issuer: totp.issuer,
        period: totp.step as i64,
        secret: Secret::Raw(totp.secret).to_encoded().to_string(),
    })
    .await?;

    Ok(())
}

pub async fn edit_issuer(access_token: AccessToken, params: EditIssuerRequestParams) -> Result<()> {
    let totp = miniprogram::totp::fetch(params.id).await?;

    totp.ensure_permission(access_token.user_id)?;

    miniprogram::totp::update_issuer(totp.id, params.issuer.as_str()).await?;

    Ok(())
}

pub async fn edit_username(
    access_token: AccessToken,
    params: EditUsernameRequestParams,
) -> Result<()> {
    let totp = miniprogram::totp::fetch(params.id).await?;

    totp.ensure_permission(access_token.user_id)?;

    miniprogram::totp::update_username(totp.id, params.username.as_str()).await?;

    Ok(())
}

pub async fn delete(access_token: AccessToken, id: i64) -> Result<()> {
    let totp = miniprogram::totp::fetch(id).await?;

    totp.ensure_permission(access_token.user_id)?;

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
