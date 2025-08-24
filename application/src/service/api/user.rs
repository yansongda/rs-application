use application_database::account::access_token;
use application_database::account::user;
use application_kernel::result::{Error, Result};

pub async fn detail(id: u64) -> Result<user::User> {
    if id <= 0 {
        return Err(Error::ParamsUserNotFound(None));
    }

    user::fetch(id).await
}

pub async fn update_avatar(
    access_token: access_token::AccessToken,
    avatar: &str,
) -> Result<user::User> {
    user::update_avatar(access_token.user_id, avatar).await
}

pub async fn update_nickname(
    access_token: access_token::AccessToken,
    nickname: &str,
) -> Result<user::User> {
    user::update_nickname(access_token.user_id, nickname).await
}

pub async fn update_slogan(
    access_token: access_token::AccessToken,
    slogan: &str,
) -> Result<user::User> {
    user::update_slogan(access_token.user_id, slogan).await
}

// todo: 对相同手机号进行账号数据合并
pub async fn update_phone(
    access_token: access_token::AccessToken,
    phone: &str,
) -> Result<user::User> {
    user::update_phone(access_token.user_id, phone).await
}
