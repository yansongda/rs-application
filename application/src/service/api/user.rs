use application_database::::access_token::AccessToken;
use application_database::::user::User;
use application_kernel::result::{Error, Result};
use crate::repository;

pub async fn detail(id: i64) -> Result<User> {
    if id <= 0 {
        return Err(Error::ParamsUserNotFound(None));
    }

    repository::user::fetch(id).await
}

pub async fn update_avatar(access_token: AccessToken, avatar: &str) -> Result<User> {
    repository::user::update_avatar(access_token.user_id, avatar).await
}

pub async fn update_nickname(access_token: AccessToken, nickname: &str) -> Result<User> {
    repository::user::update_nickname(access_token.user_id, nickname).await
}

pub async fn update_slogan(access_token: AccessToken, slogan: &str) -> Result<User> {
    repository::user::update_slogan(access_token.user_id, slogan).await
}

// todo: 对相同手机号进行账号数据合并
pub async fn update_phone(access_token: AccessToken, phone: &str) -> Result<User> {
    repository::user::update_phone(access_token.user_id, phone).await
}
