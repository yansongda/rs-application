use crate::model::wechat::miniprogram::access_token::{AccessToken, AccessTokenData};
use crate::model::result::{Error, Result};
use crate::repository::miniprogram;
use crate::service::wechat;

pub async fn login(code: &str) -> Result<AccessToken> {
    let wechat_response = wechat::login(code).await?;
    let open_id = wechat_response.openid.clone().unwrap();
    let user_id = get_login_user_id(open_id.as_str()).await?;

    let exist = miniprogram::access_token::fetch_by_user_id(user_id).await;

    if exist.is_ok() {
        return miniprogram::access_token::update(
            exist.unwrap().id,
            AccessTokenData::from(wechat_response),
        )
        .await;
    }

    match exist.unwrap_err() {
        Error::AccessTokenNotFound(_) => {
            miniprogram::access_token::insert(user_id, AccessTokenData::from(wechat_response)).await
        }
        e => Err(e),
    }
}

async fn get_login_user_id(open_id: &str) -> Result<i64> {
    let result = miniprogram::user::fetch_by_open_id(open_id).await;

    if let Ok(user) = result {
        return Ok(user.id);
    }

    match result.unwrap_err() {
        Error::UserNotFound(_) => miniprogram::user::insert(open_id).await.map(|u| u.id),
        e => Err(e),
    }
}
