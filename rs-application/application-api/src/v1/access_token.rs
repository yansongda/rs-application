use crate::request::Validator;
use crate::request::access_token::{
    LoginRefreshRequest, LoginRefreshResponse, LoginRequest, LoginResponse,
};
use crate::response::{Resp, Response, get_request_id};
use crate::service;
use salvo::{Request, handler};

#[handler]
pub async fn login(request: &mut Request) -> Resp<LoginResponse> {
    let params = request.parse_json::<LoginRequest>().await?;

    let req = params.validate()?;

    let (refresh_token, access_token) = service::access_token::login(&req).await?;

    Ok(Response::success(get_request_id(request), LoginResponse {
        access_token: access_token.access_token.to_owned(),
        expired_in: access_token.get_expired_in(),
        refresh_token: refresh_token.refresh_token,
    }))
}

#[handler]
pub async fn login_refresh(request: &mut Request) -> Resp<LoginRefreshResponse> {
    let params = request.parse_json::<LoginRefreshRequest>().await?;

    let req = params.validate()?;

    let (refresh_token, access_token) = service::access_token::login_refresh(&req).await?;

    Ok(Response::success(get_request_id(request), LoginRefreshResponse {
        access_token: access_token.access_token.to_owned(),
        expired_in: access_token.get_expired_in(),
        refresh_token: refresh_token.refresh_token,
    }))
}

#[handler]
pub async fn valid(request: &mut Request) -> Resp<()> {
    Ok(Response::success(get_request_id(request), ()))
}
