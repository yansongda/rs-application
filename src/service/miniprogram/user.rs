use crate::model::miniprogram::user::{EditUser, User};
use crate::model::result::{Error, Result};
use crate::repository::miniprogram;

pub async fn detail(id: i64) -> Result<User> {
    if id <= 0 {
        return Err(Error::ParamsMiniprogramUserNotFound(None));
    }

    miniprogram::user::fetch(id).await
}

pub async fn edit(id: i64, params: EditUser) -> Result<User> {
    if id < 0 {
        return miniprogram::user::insert(params).await;
    }

    miniprogram::user::update(id, params).await
}
