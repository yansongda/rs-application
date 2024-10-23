use axum::Extension;

use crate::miniprogram_api::extract::Json;
use crate::miniprogram_api::response::Resp;
use crate::model::miniprogram::access_token::AccessToken;
use crate::model::result::Response;
use crate::request::miniprogram::user::{DetailResponse, UpdateRequest};
use crate::request::Validator;
use crate::service;

pub async fn detail(Extension(access_token): Extension<AccessToken>) -> Resp<DetailResponse> {
    let user =
        service::miniprogram::user::detail_by_open_id(access_token.data.open_id.as_str()).await?;

    Ok(Response::success(user.into()))
}

pub async fn update(
    Extension(access_token): Extension<AccessToken>,
    Json(request): Json<UpdateRequest>,
) -> Resp<()> {
    let params = request.validate()?;

    service::miniprogram::user::update(access_token.user_id, params).await?;

    Ok(Response::success(()))
}
