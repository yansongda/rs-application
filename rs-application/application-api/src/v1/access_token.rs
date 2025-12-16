use salvo::{handler, Request};
use crate::request::Validator;
use crate::request::access_token::{
    LoginRefreshRequest, LoginRefreshResponse, LoginRequest, LoginResponse,
};
use crate::response::{Resp, Response};
use crate::service;
use application_database::account::access_token::AccessToken;

#[handler]
pub async fn login(&self, request: &mut Request) -> Resp<LoginResponse> {
    let req = request.validate()?;

    let (refresh_token, access_token) = service::access_token::login(&req).await?;

    Ok(Response::success(LoginResponse {
        access_token: access_token.access_token.to_owned(),
        expired_in: access_token.get_expired_in(),
        refresh_token: refresh_token.refresh_token,
    }))
}

pub async fn login_refresh(Json(request): Json<LoginRefreshRequest>) -> Resp<LoginRefreshResponse> {
    let req = request.validate()?;

    let (refresh_token, access_token) = service::access_token::login_refresh(&req).await?;

    Ok(Response::success(LoginRefreshResponse {
        access_token: access_token.access_token.to_owned(),
        expired_in: access_token.get_expired_in(),
        refresh_token: refresh_token.refresh_token,
    }))
}

pub async fn valid(Extension(_): Extension<AccessToken>) -> Resp<()> {
    Ok(Response::success(()))
}
