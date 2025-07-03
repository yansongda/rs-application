use application_database::entity::account::access_token::AccessToken;
use application_database::entity::tool::totp::{CreatedTotp, TotpConfig};
use application_kernel::result::{Error, Result};
use application_database::repository;
use crate::request::api::totp::{
    DetailResponse, EditIssuerRequestParams, EditUsernameRequestParams,
};
use totp_rs::{Secret, TOTP};
use tracing::error;

pub async fn all(access_token: AccessToken) -> Result<Vec<DetailResponse>> {
    let totp = repository::tool::totp::all(access_token.user_id).await?;

    Ok(totp.into_iter().map(|t| t.into()).collect())
}

pub async fn detail(access_token: AccessToken, id: i64) -> Result<DetailResponse> {
    let totp = repository::tool::totp::fetch(id).await?;

    totp.ensure_permission(access_token.user_id)?;

    Ok(totp.into())
}

pub async fn create(access_token: AccessToken, uri: String) -> Result<()> {
    let totp = TOTP::from_url_unchecked(uri.as_str()).map_err(|e| {
        error!("TOTP 链接解析失败: {}", e);

        Error::ParamsTotpParseFailed(None)
    })?;

    repository::tool::totp::insert(CreatedTotp {
        user_id: access_token.user_id,
        username: totp.account_name,
        issuer: totp.issuer,
        config: TotpConfig {
            period: totp.step,
            secret: Secret::Raw(totp.secret).to_encoded().to_string(),
        },
    })
    .await?;

    Ok(())
}

pub async fn edit_issuer(access_token: AccessToken, params: EditIssuerRequestParams) -> Result<()> {
    let totp = repository::tool::totp::fetch(params.id).await?;

    totp.ensure_permission(access_token.user_id)?;

    repository::tool::totp::update_issuer(totp.id, params.issuer.as_str()).await?;

    Ok(())
}

pub async fn edit_username(
    access_token: AccessToken,
    params: EditUsernameRequestParams,
) -> Result<()> {
    let totp = repository::tool::totp::fetch(params.id).await?;

    totp.ensure_permission(access_token.user_id)?;

    repository::tool::totp::update_username(totp.id, params.username.as_str()).await?;

    Ok(())
}

pub async fn delete(access_token: AccessToken, id: i64) -> Result<()> {
    let totp = repository::tool::totp::fetch(id).await?;

    totp.ensure_permission(access_token.user_id)?;

    repository::tool::totp::delete(id).await?;

    Ok(())
}
