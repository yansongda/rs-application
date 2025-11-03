use crate::request::totp::{DetailResponse, EditIssuerRequestParams, EditUsernameRequestParams};
use application_database::account::access_token;
use application_database::tool::totp;
use application_kernel::result::{Error, Result};
use totp_rs::{Secret, TOTP};
use tracing::error;

pub async fn all(access_token: access_token::AccessToken) -> Result<Vec<DetailResponse>> {
    let totp = totp::all(access_token.user_id).await?;

    Ok(totp.into_iter().map(|t| t.into()).collect())
}

pub async fn detail(access_token: access_token::AccessToken, id: u64) -> Result<DetailResponse> {
    let totp = totp::fetch(id).await?;

    totp.ensure_permission(access_token.user_id)?;

    Ok(totp.into())
}

pub async fn create(
    access_token: access_token::AccessToken,
    uri: String,
) -> Result<DetailResponse> {
    let totp = TOTP::from_url_unchecked(uri.as_str()).map_err(|e| {
        error!("TOTP 链接解析失败: {}", e);

        Error::ParamsTotpParseFailed(None)
    })?;

    Ok(totp::insert(totp::CreatedTotp {
        user_id: access_token.user_id,
        username: totp.account_name,
        issuer: totp.issuer,
        config: totp::TotpConfig {
            period: totp.step,
            secret: Secret::Raw(totp.secret).to_encoded().to_string(),
        },
    })
    .await?
    .into())
}

pub async fn edit_issuer(
    access_token: access_token::AccessToken,
    params: EditIssuerRequestParams,
) -> Result<()> {
    let totp = totp::fetch(params.id).await?;

    totp.ensure_permission(access_token.user_id)?;

    totp::update_issuer(totp.id, params.issuer.as_str()).await?;

    Ok(())
}

pub async fn edit_username(
    access_token: access_token::AccessToken,
    params: EditUsernameRequestParams,
) -> Result<()> {
    let totp = totp::fetch(params.id).await?;

    totp.ensure_permission(access_token.user_id)?;

    totp::update_username(totp.id, params.username.as_str()).await?;

    Ok(())
}

pub async fn delete(access_token: access_token::AccessToken, id: u64) -> Result<()> {
    let totp = totp::fetch(id).await?;

    totp.ensure_permission(access_token.user_id)?;

    totp::delete(id).await?;

    Ok(())
}
