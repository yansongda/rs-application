use crate::request::api::access_token::LoginRequest;
use application_database::account::Platform;
use application_database::account::access_token;
use application_database::account::third_user;
use application_database::account::user;
use application_kernel::result::{Error, Result};
use application_util::wechat;

pub async fn login(request: LoginRequest) -> Result<access_token::AccessToken> {
    let platform = request.platform.unwrap();
    let (user_id, access_token_data) = match platform {
        Platform::Wechat => login_wechat(request.third_id.unwrap().as_str(), request.code.unwrap().as_str()).await,
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

async fn login_wechat(third_id: &str, code: &str) -> Result<(u64, access_token::AccessTokenData)> {
    let wechat_response = wechat::login(code).await?;
    let open_id = wechat_response.get_open_id().unwrap();

    Ok((
        get_third_user_id(&Platform::Wechat, open_id).await?,
        access_token::AccessTokenData::from(wechat_response),
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
