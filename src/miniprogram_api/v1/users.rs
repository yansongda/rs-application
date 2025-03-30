use axum::Extension;

use crate::miniprogram_api::extract::Json;
use crate::miniprogram_api::response::Resp;
use crate::model::miniprogram::access_token::AccessToken;
use crate::model::result::Response;
use crate::request::Validator;
use crate::request::miniprogram::user::{DetailResponse, EditRequest};
use crate::service;

pub async fn detail(Extension(access_token): Extension<AccessToken>) -> Resp<DetailResponse> {
    let user = service::miniprogram::user::detail(access_token.user_id).await?;

    Ok(Response::success(user.into()))
}

pub async fn edit(
    Extension(access_token): Extension<AccessToken>,
    Json(request): Json<EditRequest>,
) -> Resp<()> {
    let params = request.validate()?;

    service::miniprogram::user::edit(access_token.user_id, params).await?;

    Ok(Response::success(()))
}
