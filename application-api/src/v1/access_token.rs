use crate::extract::Json;
use crate::request::Validator;
use crate::request::access_token::{
    LoginRequest, LoginResponse, RefreshLoginRequest, RefreshLoginResponse,
};
use crate::response::{Resp, Response};
use crate::service;
use application_database::account::access_token::AccessToken;
use application_kernel::config::G_CONFIG;
use axum::Extension;

pub async fn login(Json(request): Json<LoginRequest>) -> Resp<LoginResponse> {
    let req = request.validate()?;

    let (refresh_token, access_token) = service::access_token::login(&req).await?;

    Ok(Response::success(LoginResponse {
        access_token: access_token.access_token,
        expired_in: G_CONFIG.access_token.expired_in,
        refresh_token: refresh_token.refresh_token,
    }))
}

pub async fn login_refresh(Json(request): Json<RefreshLoginRequest>) -> Resp<RefreshLoginResponse> {
    let req = request.validate()?;

    let token = service::access_token::login_refresh(&req).await?;

    Ok(Response::success(token.into()))
}

pub async fn valid(Extension(_): Extension<AccessToken>) -> Resp<()> {
    Ok(Response::success(()))
}
