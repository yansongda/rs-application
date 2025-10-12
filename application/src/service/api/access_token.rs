use crate::request::api::access_token::LoginRequest;
use application_database::account::access_token;
use application_database::account::third_user;
use application_database::account::user;
use application_database::account::{Platform, third_config};
use application_kernel::result::{Error, Result};
use application_util::{huawei, wechat};

pub async fn login(request: &LoginRequest) -> Result<access_token::AccessToken> {
    let platform = request.platform.unwrap();
    let (user_id, access_token_data) = match platform {
        Platform::Wechat => login_wechat(request).await,
        Platform::Huawei => login_huawei(request).await,
        _ => Err(Error::ParamsLoginPlatformUnsupported(None)),
    }?;

    let exist = access_token::fetch_by_user_id(user_id).await;

    if exist.is_ok() {
        return access_token::update(exist?, access_token_data).await;
    }

    match exist.unwrap_err() {
        Error::ParamsAccessTokenNotFound(_) => {
            access_token::insert(&platform, user_id, access_token_data).await
        }
        e => Err(e),
    }
}

async fn login_wechat(request: &LoginRequest) -> Result<(u64, access_token::AccessTokenData)> {
    let third_id = request
        .third_id
        .as_ref()
        .ok_or(Error::ParamsLoginPlatformThirdIdFormatInvalid(None))?;

    let code = request
        .code
        .as_ref()
        .ok_or(Error::ParamsLoginCodeFormatInvalid(None))?;

    let config = third_config::fetch(&Platform::Wechat, third_id).await?;

    let app_secret = config
        .config
        .as_ref()
        .and_then(|c| c.wechat.as_ref())
        .ok_or(Error::InternalDatabaseDataInvalid(None))?
        .app_secret
        .as_ref();

    let wechat_response =
        wechat::login(code.as_str(), config.third_id.as_str(), app_secret).await?;

    Ok((
        get_third_user_id(&Platform::Wechat, wechat_response.openid.as_str()).await?,
        access_token::AccessTokenData::from(wechat_response),
    ))
}

async fn login_huawei(request: &LoginRequest) -> Result<(u64, access_token::AccessTokenData)> {
    let third_id = request
        .third_id
        .as_ref()
        .ok_or(Error::ParamsLoginPlatformThirdIdFormatInvalid(None))?;

    let code = request
        .code
        .as_ref()
        .ok_or(Error::ParamsLoginCodeFormatInvalid(None))?;

    let config = third_config::fetch(&Platform::Huawei, third_id).await?;

    let app_secret = config
        .config
        .as_ref()
        .and_then(|c| c.huawei.as_ref())
        .ok_or(Error::InternalDatabaseDataInvalid(None))?
        .client_secret
        .as_ref();

    let token_response = huawei::token(code.as_str(), config.third_id.as_str(), app_secret).await?;
    let token_info_response = huawei::token_info(token_response.access_token.as_str()).await?;

    Ok((
        get_third_user_id(&Platform::Huawei, token_info_response.union_id.as_str()).await?,
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

async fn get_third_user_id(platform: &Platform, third_id: &str) -> Result<u64> {
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
