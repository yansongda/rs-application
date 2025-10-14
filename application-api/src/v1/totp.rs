use axum::Extension;

use crate::extract::Json;
use crate::request::Validator;
use crate::request::totp::{
    CreateRequest, DeleteRequest, DetailRequest, DetailResponse, EditIssuerRequest,
    EditUsernameRequest,
};
use crate::response::Resp;
use crate::response::Response;
use crate::service;
use application_database::account::access_token::AccessToken;

pub async fn all(Extension(access_token): Extension<AccessToken>) -> Resp<Vec<DetailResponse>> {
    Ok(Response::success(service::totp::all(access_token).await?))
}

pub async fn detail(
    Extension(access_token): Extension<AccessToken>,
    Json(request): Json<DetailRequest>,
) -> Resp<DetailResponse> {
    let id = request.validate()?;

    Ok(Response::success(
        service::totp::detail(access_token, id).await?,
    ))
}

pub async fn create(
    Extension(access_token): Extension<AccessToken>,
    Json(request): Json<CreateRequest>,
) -> Resp<()> {
    let uri = request.validate()?;

    service::totp::create(access_token, uri).await?;

    Ok(Response::success(()))
}

pub async fn edit_issuer(
    Extension(access_token): Extension<AccessToken>,
    Json(request): Json<EditIssuerRequest>,
) -> Resp<()> {
    let params = request.validate()?;

    service::totp::edit_issuer(access_token, params).await?;

    Ok(Response::success(()))
}

pub async fn edit_username(
    Extension(access_token): Extension<AccessToken>,
    Json(request): Json<EditUsernameRequest>,
) -> Resp<()> {
    let params = request.validate()?;

    service::totp::edit_username(access_token, params).await?;

    Ok(Response::success(()))
}

pub async fn delete(
    Extension(access_token): Extension<AccessToken>,
    Json(request): Json<DeleteRequest>,
) -> Resp<()> {
    let id = request.validate()?;

    service::totp::delete(access_token, id).await?;

    Ok(Response::success(()))
}
