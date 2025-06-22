use crate::model::entity::access_token::{AccessToken, AccessTokenData};
use crate::model::entity::third_user::Platform;
use crate::model::entity::user::Config;
use crate::model::result::{Error, Result};
use crate::repository;
use crate::request::api::access_token::LoginRequest;
use crate::service::wechat;

pub async fn login(request: LoginRequest) -> Result<AccessToken> {
    let platform = request.platform.unwrap();
    let (user_id, access_token_data) = match platform {
        Platform::Wechat => login_wechat(request.code.unwrap().as_str()).await,
        _ => Err(Error::ParamsMiniprogramLoginPlatformUnsupported(None)),
    }?;

    let exist = repository::access_token::fetch_by_user_id(user_id).await;

    if exist.is_ok() {
        return repository::access_token::update(exist?.id, &access_token_data).await;
    }

    match exist.unwrap_err() {
        Error::ParamsMiniprogramAccessTokenNotFound(_) => {
            repository::access_token::insert(platform, user_id, &access_token_data).await
        }
        e => Err(e),
    }
}

async fn login_wechat(code: &str) -> Result<(i64, AccessTokenData)> {
    let wechat_response = wechat::login(code).await?;
    let open_id = wechat_response.openid.clone().unwrap();

    Ok((
        get_third_user_id(Platform::Wechat, open_id.as_str()).await?,
        AccessTokenData::from(wechat_response),
    ))
}

async fn get_third_user_id(platform: Platform, third_id: &str) -> Result<i64> {
    let result = repository::third_user::fetch(&platform, third_id).await;

    if let Ok(user) = result {
        return Ok(user.user_id);
    }

    match result.unwrap_err() {
        Error::ParamsMiniprogramThirdUserNotFound(_) => {
            let user = repository::user::insert(
                None,
                Config {
                    avatar: None,
                    nickname: None,
                    slogan: None,
                },
            )
            .await?;

            repository::third_user::insert(platform, third_id, user.id).await?;

            Ok(user.id)
        }
        e => Err(e),
    }
}
