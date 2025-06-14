use crate::model::miniprogram::access_token::AccessToken;
use crate::model::miniprogram::user::{Config, User};
use crate::model::result::{Error, Result};
use crate::repository::miniprogram;

pub async fn detail(id: i64) -> Result<User> {
    if id <= 0 {
        return Err(Error::ParamsMiniprogramUserNotFound(None));
    }

    miniprogram::user::fetch(id).await
}

pub async fn update_avatar(access_token: AccessToken, avatar: &str) -> Result<User> {
    if access_token.user_id > 0 {
        return miniprogram::user::update_avatar(access_token.user_id, avatar).await;
    }

    let user = miniprogram::user::insert(
        None,
        Config {
            avatar: Some(avatar.to_string()),
            nickname: None,
            slogan: None,
        },
    )
    .await?;

    miniprogram::access_token::update_user_id(access_token.id, user.id).await?;

    Ok(user)
}

pub async fn update_nickname(access_token: AccessToken, nickname: &str) -> Result<User> {
    if access_token.user_id > 0 {
        return miniprogram::user::update_nickname(access_token.user_id, nickname).await;
    }

    let user = miniprogram::user::insert(
        None,
        Config {
            avatar: None,
            nickname: Some(nickname.to_string()),
            slogan: None,
        },
    )
    .await?;

    miniprogram::access_token::update_user_id(access_token.id, user.id).await?;

    Ok(user)
}

pub async fn update_slogan(access_token: AccessToken, slogan: &str) -> Result<User> {
    if access_token.user_id > 0 {
        return miniprogram::user::update_slogan(access_token.user_id, slogan).await;
    }

    let user = miniprogram::user::insert(
        None,
        Config {
            avatar: None,
            nickname: None,
            slogan: Some(slogan.to_string()),
        },
    )
    .await?;

    miniprogram::access_token::update_user_id(access_token.id, user.id).await?;

    Ok(user)
}

// todo: 对相同手机号进行账号数据合并
pub async fn update_phone(access_token: AccessToken, phone: &str) -> Result<User> {
    if access_token.user_id > 0 {
        return miniprogram::user::update_phone(access_token.user_id, phone).await;
    }

    let user = miniprogram::user::insert(
        Some(phone),
        Config {
            avatar: None,
            nickname: None,
            slogan: None,
        },
    )
    .await?;

    miniprogram::access_token::update_user_id(access_token.id, user.id).await?;

    Ok(user)
}
