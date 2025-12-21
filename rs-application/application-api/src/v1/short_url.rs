use crate::request::Validator;
use crate::request::short_url::{CreateRequest, CreateResponse, DetailRequest, DetailResponse};
use crate::response::Resp;
use crate::response::Response;
use crate::response::Result;
use crate::service;
use salvo::prelude::Redirect;
use salvo::{Request, handler};

#[handler]
pub async fn create(request: &mut Request) -> Resp<CreateResponse> {
    let params = request.parse_json::<CreateRequest>().await?;

    let short_url = service::short_url::create(params.validate()?.as_str()).await?;

    Ok(Response::success(CreateResponse::from(short_url)))
}

#[handler]
pub async fn detail(request: &mut Request) -> Resp<DetailResponse> {
    let params = request.parse_json::<DetailRequest>().await?;

    let short_url = service::short_url::detail(params.validate()?.as_str()).await?;

    Ok(Response::success(DetailResponse::from(short_url)))
}

#[handler]
pub async fn redirect(request: &mut Request, response: &mut salvo::Response) -> Result<()> {
    let short: String = request.param("short").unwrap_or_default();

    let short_url = service::short_url::detail(short.as_str()).await?;

    response.render(Redirect::found(short_url.url.as_str()));

    Ok(())
}
