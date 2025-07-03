use axum::Extension;

use crate::api::extract::Json;
use crate::api::response::Resp;
use crate::request::Validator;
use crate::request::api::user::{
    DetailResponse, EditAvatarRequest, EditNicknameRequest, EditPhoneRequest, EditSloganRequest,
};
use crate::service;
use application_database::account::access_token::AccessToken;
use application_kernel::result::Response;

pub async fn detail(Extension(access_token): Extension<AccessToken>) -> Resp<DetailResponse> {
    let user = service::api::user::detail(access_token.user_id).await?;

    Ok(Response::success(user.into()))
}

pub async fn edit_avatar(
    Extension(access_token): Extension<AccessToken>,
    Json(request): Json<EditAvatarRequest>,
) -> Resp<()> {
    let avatar = request.validate()?;

    service::api::user::update_avatar(access_token, &avatar).await?;

    Ok(Response::success(()))
}

pub async fn edit_nickname(
    Extension(access_token): Extension<AccessToken>,
    Json(request): Json<EditNicknameRequest>,
) -> Resp<()> {
    let nickname = request.validate()?;

    service::api::user::update_nickname(access_token, &nickname).await?;

    Ok(Response::success(()))
}

pub async fn edit_slogan(
    Extension(access_token): Extension<AccessToken>,
    Json(request): Json<EditSloganRequest>,
) -> Resp<()> {
    let slogan = request.validate()?;

    service::api::user::update_slogan(access_token, &slogan).await?;

    Ok(Response::success(()))
}

pub async fn edit_phone(
    Extension(access_token): Extension<AccessToken>,
    Json(request): Json<EditPhoneRequest>,
) -> Resp<()> {
    let phone = request.validate()?;

    service::api::user::update_phone(access_token, &phone).await?;

    Ok(Response::success(()))
}
