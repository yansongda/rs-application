use crate::api::extract::Json;
use crate::api::response::Resp;
use crate::request::Validator;
use crate::request::api::access_token::{LoginRequest, LoginResponse};
use crate::service;
use application_database::account::access_token::AccessToken;
use application_kernel::result::Response;
use axum::Extension;

pub async fn login(Json(request): Json<LoginRequest>) -> Resp<LoginResponse> {
    let req = request.validate()?;

    let token = service::api::access_token::login(req).await?;

    Ok(Response::success(token.into()))
}

pub async fn valid(Extension(_): Extension<AccessToken>) -> Resp<()> {
    Ok(Response::success(()))
}
