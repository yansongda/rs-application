use crate::model::miniprogram::access_token::AccessToken;
use crate::model::miniprogram::user::User;
use crate::model::result::{Error, Result};
use crate::repository::miniprogram;

pub async fn detail(id: i64) -> Result<User> {
    if id <= 0 {
        return Err(Error::ParamsMiniprogramUserNotFound(None));
    }

    miniprogram::user::fetch(id).await
}

pub async fn update_avatar(access_token: AccessToken, avatar: &str) -> Result<User> {
    miniprogram::user::update_avatar(access_token.user_id, avatar).await
}

pub async fn update_nickname(access_token: AccessToken, nickname: &str) -> Result<User> {
    miniprogram::user::update_nickname(access_token.user_id, nickname).await
}

pub async fn update_slogan(access_token: AccessToken, slogan: &str) -> Result<User> {
    miniprogram::user::update_slogan(access_token.user_id, slogan).await
}

// todo: 对相同手机号进行账号数据合并
pub async fn update_phone(access_token: AccessToken, phone: &str) -> Result<User> {
    miniprogram::user::update_phone(access_token.user_id, phone).await
}
