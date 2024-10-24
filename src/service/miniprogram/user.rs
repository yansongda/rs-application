use crate::model::wechat::miniprogram::user::{UpdateUser, User};
use crate::model::result::Result;
use crate::repository::miniprogram;

pub async fn detail_by_open_id(open_id: &str) -> Result<User> {
    miniprogram::user::fetch_by_open_id(open_id).await
}

pub async fn update(id: i64, update_user: UpdateUser) -> Result<User> {
    miniprogram::user::update(id, update_user).await
}
