use axum::extract::Path;
use axum::response::Redirect;

use crate::api::extract::Json;
use crate::api::response::Resp;
use crate::api::response::Response;
use crate::request::Validator;
use crate::request::api::short_url::{
    CreateRequest, CreateResponse, DetailRequest, DetailResponse,
};
use crate::service;
use application_kernel::result::Result;

pub async fn create(Json(request): Json<CreateRequest>) -> Resp<CreateResponse> {
    let url = request.validate()?;

    let short_url = service::api::short_url::create(&url).await?;

    Ok(Response::success(CreateResponse::from(short_url)))
}

pub async fn detail(Json(request): Json<DetailRequest>) -> Resp<DetailResponse> {
    let short = request.validate()?;

    let short_url = service::api::short_url::detail(&short).await?;

    Ok(Response::success(DetailResponse::from(short_url)))
}

pub async fn redirect(Path(short): Path<String>) -> Result<Redirect> {
    let short_url = service::api::short_url::detail(short.as_str()).await?;

    Ok(Redirect::temporary(short_url.url.as_str()))
}
