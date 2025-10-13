use crate::api::extract::Json;
use crate::api::response::Resp;
use crate::api::response::Response;
use crate::request::Validator;
use crate::request::api::access_token::{LoginRequest, LoginResponse, RefreshLoginRequest};
use crate::service;
use application_database::account::access_token::AccessToken;
use axum::Extension;

pub async fn login(Json(request): Json<LoginRequest>) -> Resp<LoginResponse> {
    let req = request.validate()?;

    let token = service::api::access_token::login(&req).await?;

    Ok(Response::success(LoginResponse {
        access_token: token.access_token,
        expired_at: token.expired_at,
        refresh_token: token.refresh_token,
    }))
}

pub async fn refresh_login(Json(request): Json<RefreshLoginRequest>) -> Resp<LoginResponse> {
    let req = request.validate()?;
    
    let token = service::api::access_token::refresh_login(&req).await?;

    Ok(Response::success(new_token.into()))
}

pub async fn valid(Extension(_): Extension<AccessToken>) -> Resp<()> {
    Ok(Response::success(()))
}
