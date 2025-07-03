use application_database::entity::account::access_token::{AccessToken, AccessTokenData};
use application_database::entity::account::third_user::Platform;
use application_database::entity::account::user::Config;
use application_kernel::result::{Error, Result};
use application_database::repository;
use crate::request::api::access_token::LoginRequest;
use application_util::wechat;

pub async fn login(request: LoginRequest) -> Result<AccessToken> {
    let platform = request.platform.unwrap();
    let (user_id, access_token_data) = match platform {
        Platform::Wechat => login_wechat(request.code.unwrap().as_str()).await,
        _ => Err(Error::ParamsLoginPlatformUnsupported(None)),
    }?;

    let exist = repository::account::access_token::fetch_by_user_id(user_id).await;

    if exist.is_ok() {
        return repository::account::access_token::update(exist?, &access_token_data).await;
    }

    match exist.unwrap_err() {
        Error::ParamsAccessTokenNotFound(_) => {
            repository::account::access_token::insert(platform, user_id, &access_token_data).await
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
    let result = repository::account::third_user::fetch(&platform, third_id).await;

    if let Ok(user) = result {
        return Ok(user.user_id);
    }

    match result.unwrap_err() {
        Error::ParamsThirdUserNotFound(_) => {
            let user = repository::account::user::insert(
                None,
                Config {
                    avatar: None,
                    nickname: None,
                    slogan: None,
                },
            )
            .await?;

            repository::account::third_user::insert(platform, third_id, user.id).await?;

            Ok(user.id)
        }
        e => Err(e),
    }
}
