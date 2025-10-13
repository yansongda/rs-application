use crate::request::api::access_token::{LoginRequestParams, RefreshLoginRequestParams};
use application_database::account::third_user;
use application_database::account::user;
use application_database::account::{Platform, third_config};
use application_database::account::{access_token, refresh_token};
use application_kernel::result::{Error, Result};
use application_util::{huawei, wechat};

pub async fn login(
    request: &LoginRequestParams,
) -> Result<(refresh_token::RefreshToken, access_token::AccessToken)> {
    let platform = request.platform;
    let (user_id, access_token_data) = match platform {
        Platform::Wechat => login_wechat(request).await,
        Platform::Huawei => login_huawei(request).await,
        _ => Err(Error::ParamsLoginPlatformUnsupported(None)),
    }?;

    let access_token =
        access_token::update_or_insert(&platform, user_id, access_token_data).await?;

    Ok((refresh_token::insert(access_token.id).await?, access_token))
}

pub async fn refresh_login(
    request: &RefreshLoginRequestParams,
) -> Result<access_token::AccessToken> {
    let platform = request.platform;
    let exist =
        access_token::fetch_by_refresh_token(&platform, request.refresh_token.as_str()).await?;

    let (user_id, access_token_data) = match platform {
        Platform::Wechat => {
            login_wechat(&LoginRequestParams {
                platform,
                third_id: request.third_id.clone(),
                code: exist
                    .wechat
                    .as_ref()
                    .map(|t| t.code.clone())
                    .unwrap_or_default(),
            })
            .await
        }
        Platform::Huawei => {
            login_huawei(&LoginRequestParams {
                platform,
                third_id: request.third_id.clone(),
                code: exist
                    .huawei
                    .as_ref()
                    .map(|t| t.access_token.clone())
                    .unwrap_or_default(),
            })
            .await
        }
        _ => Err(Error::ParamsLoginPlatformUnsupported(None)),
    }?;

    access_token::update(exist, access_token_data).await
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

async fn login_huawei(
    request: &LoginRequestParams,
) -> Result<(u64, access_token::AccessTokenData)> {
    let config = third_config::fetch(&Platform::Huawei, request.third_id.as_str()).await?;

    let app_secret = config
        .config
        .as_ref()
        .and_then(|c| c.huawei.as_ref())
        .ok_or(Error::InternalDatabaseDataInvalid(None))?
        .client_secret
        .as_ref();

    let token_response =
        huawei::token(request.code.as_str(), config.third_id.as_str(), app_secret).await?;
    let token_info_response = huawei::token_info(token_response.access_token.as_str()).await?;

    Ok((
        get_user_id(&Platform::Huawei, token_info_response.union_id.as_str()).await?,
        access_token::AccessTokenData {
            wechat: None,
            huawei: Some(access_token::HuaweiAccessTokenData {
                token_type: token_response.token_type,
                access_token: token_response.access_token,
                scope: token_response.scope,
                refresh_token: token_response.refresh_token,
                client_id: token_info_response.client_id,
                expires_in: token_info_response.expires_in,
                union_id: token_info_response.union_id,
                project_id: token_info_response.project_id,
                r#type: token_info_response.r#type,
            }),
        },
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
