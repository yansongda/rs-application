use crate::miniprogram_api::extract::Json;
use crate::miniprogram_api::response::Resp;
use crate::model::miniprogram::access_token::AccessToken;
use crate::model::result::Response;
use crate::request::Validator;
use crate::request::miniprogram::access_token::{LoginRequest, LoginResponse};
use crate::service;
use axum::Extension;

pub async fn login(Json(request): Json<LoginRequest>) -> Resp<LoginResponse> {
    let req = request.validate()?;

    let token = service::miniprogram::access_token::login(req).await?;

    Ok(Response::success(token.into()))
}

pub async fn valid(Extension(_): Extension<AccessToken>) -> Resp<()> {
    Ok(Response::success(()))
}
