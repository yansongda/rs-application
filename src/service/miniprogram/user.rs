use crate::model::miniprogram::user::{UpdateUser, User};
use crate::model::result::Result;
use crate::repository::miniprogram;

pub async fn detail(user_id: i64) -> Result<User> {
    miniprogram::user::fetch(user_id).await
}

pub async fn update(id: i64, update_user: UpdateUser) -> Result<User> {
    miniprogram::user::update(id, update_user).await
}
