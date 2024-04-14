use axum::Extension;

use crate::api::extract::Json;
use crate::api::response::Resp;
use crate::model::result::Response;
use crate::model::user::User;
use crate::request::totp::{
    CreateRequest, DeleteRequest, DetailRequest, DetailResponse, UpdateRequest,
};
use crate::request::Validator;
use crate::service;

pub async fn all(Extension(current_user): Extension<User>) -> Resp<Vec<DetailResponse>> {
    Ok(Response::success(service::totp::all(current_user).await?))
}

pub async fn detail(
    Extension(current_user): Extension<User>,
    Json(request): Json<DetailRequest>,
) -> Resp<DetailResponse> {
    let id = request.validate()?;

    Ok(Response::success(
        service::totp::detail(current_user, id).await?,
    ))
}

pub async fn create(
    Extension(current_user): Extension<User>,
    Json(request): Json<CreateRequest>,
) -> Resp<()> {
    let uri = request.validate()?;

    service::totp::create(current_user, uri).await?;

    Ok(Response::success(()))
}

pub async fn update(
    Extension(current_user): Extension<User>,
    Json(request): Json<UpdateRequest>,
) -> Resp<()> {
    let params = request.validate()?;

    service::totp::update(current_user, params).await?;

    Ok(Response::success(()))
}

pub async fn delete(
    Extension(current_user): Extension<User>,
    Json(request): Json<DeleteRequest>,
) -> Resp<()> {
    let id = request.validate()?;

    service::totp::delete(current_user, id).await?;

    Ok(Response::success(()))
}
