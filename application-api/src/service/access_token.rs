use crate::request::access_token::{LoginRequestParams, RefreshLoginRequestParams};
use application_database::account::third_user;
use application_database::account::user;
use application_database::account::{Platform, third_config};
use application_database::account::{access_token, refresh_token};
use application_kernel::result::{Error, Result};
use application_util::wechat;

pub async fn login(
    request: &LoginRequestParams,
) -> Result<(refresh_token::RefreshToken, access_token::AccessToken)> {
    let platform = request.platform;
    let (user_id, access_token_data) = match platform {
        Platform::Wechat => login_wechat(request).await,
        _ => Err(Error::ParamsLoginPlatformUnsupported(None)),
    }?;

    let access_token = access_token::update_or_insert(
        &platform,
        request.third_id.as_str(),
        user_id,
        access_token_data,
    )
    .await?;

    Ok((
        refresh_token::update_or_insert(access_token.id).await?,
        access_token,
    ))
}

pub async fn login_refresh(
    request: &RefreshLoginRequestParams,
) -> Result<access_token::AccessToken> {
    let refresh_token = refresh_token::fetch(request.refresh_token.as_str())
        .await
        .map_err(|_| Error::AuthorizationRefreshTokenInvalid(None))?;

    if refresh_token.is_expired() {
        return Err(Error::AuthorizationRefreshTokenExpired(None));
    }

    let access_token = refresh_token.access_token().await?;

    if access_token.platform != request.platform || access_token.third_id != request.third_id {
        return Err(Error::AuthorizationPermissionUngranted(None));
    }

    let data = access_token.data.0.clone();

    access_token::update(access_token, data).await
}

async fn login_wechat(
    request: &LoginRequestParams,
) -> Result<(u64, access_token::AccessTokenData)> {
    let config = third_config::fetch(&Platform::Wechat, request.third_id.as_str()).await?;

    let app_secret = config
        .config
        .as_ref()
        .and_then(|c| c.wechat.as_ref())
        .ok_or(Error::InternalDatabaseDataInvalid(None))?
        .app_secret
        .as_ref();

    let wechat_response =
        wechat::login(request.code.as_str(), config.third_id.as_str(), app_secret).await?;

    Ok((
        get_user_id(&Platform::Wechat, wechat_response.openid.as_str()).await?,
        access_token::AccessTokenData::from(wechat_response),
    ))
}

async fn get_user_id(platform: &Platform, third_id: &str) -> Result<u64> {
    let result = third_user::fetch(platform, third_id).await;

    if let Ok(user) = result {
        return Ok(user.user_id);
    }

    match result.unwrap_err() {
        Error::ParamsThirdUserNotFound(_) => {
            let user_id = user::insert(
                None,
                user::Config {
                    avatar: None,
                    nickname: None,
                    slogan: None,
                },
            )
            .await?;

            third_user::insert(platform, third_id, user_id).await?;

            Ok(user_id)
        }
        e => Err(e),
    }
}
