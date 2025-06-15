use axum::Extension;

use crate::miniprogram_api::extract::Json;
use crate::miniprogram_api::response::Resp;
use crate::model::miniprogram::access_token::AccessToken;
use crate::model::result::Response;
use crate::request::Validator;
use crate::request::miniprogram::totp::{
    CreateRequest, DeleteRequest, DetailRequest, DetailResponse, EditIssuerRequest,
    EditUsernameRequest,
};
use crate::service;

pub async fn all(Extension(access_token): Extension<AccessToken>) -> Resp<Vec<DetailResponse>> {
    Ok(Response::success(
        service::miniprogram::totp::all(access_token).await?,
    ))
}

pub async fn detail(
    Extension(access_token): Extension<AccessToken>,
    Json(request): Json<DetailRequest>,
) -> Resp<DetailResponse> {
    let id = request.validate()?;

    Ok(Response::success(
        service::miniprogram::totp::detail(access_token, id).await?,
    ))
}

pub async fn create(
    Extension(access_token): Extension<AccessToken>,
    Json(request): Json<CreateRequest>,
) -> Resp<()> {
    let uri = request.validate()?;

    service::miniprogram::totp::create(access_token, uri).await?;

    Ok(Response::success(()))
}

pub async fn edit_issuer(
    Extension(access_token): Extension<AccessToken>,
    Json(request): Json<EditIssuerRequest>,
) -> Resp<()> {
    let params = request.validate()?;

    service::miniprogram::totp::edit_issuer(access_token, params).await?;

    Ok(Response::success(()))
}

pub async fn edit_username(
    Extension(access_token): Extension<AccessToken>,
    Json(request): Json<EditUsernameRequest>,
) -> Resp<()> {
    let params = request.validate()?;

    service::miniprogram::totp::edit_username(access_token, params).await?;

    Ok(Response::success(()))
}

pub async fn delete(
    Extension(access_token): Extension<AccessToken>,
    Json(request): Json<DeleteRequest>,
) -> Resp<()> {
    let id = request.validate()?;

    service::miniprogram::totp::delete(access_token, id).await?;

    Ok(Response::success(()))
}
