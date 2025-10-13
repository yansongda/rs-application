use application_database::account::access_token;
use axum::body::{Body, Bytes};
use http_body_util::BodyExt;
use std::time::Instant;

use crate::api::response::ApiErr;
use application_kernel::result::{Error, Result};
use axum::extract::Request;
use axum::http::header::CONTENT_TYPE;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use tracing::info;

pub async fn authorization(mut request: Request, next: Next) -> Response {
    let authorization = request.headers().get("Authorization");

    if authorization.is_none() {
        return ApiErr(Error::AuthorizationHeaderMissing(None)).into_response();
    }

    let auth = authorization.unwrap().to_str();

    if auth.is_err() {
        return ApiErr(Error::AuthorizationInvalidFormat(None)).into_response();
    }

    let access_token= access_token::fetch(auth.unwrap().replace("Bearer ", "").as_str()).await;

    if let Some(access_token) = access_token {
        if access_token.is_expired() {
            return ApiErr(Error::AuthorizationAccessTokenExpired(None)).into_response();
        }

        request.extensions_mut().insert(access_token.unwrap());

        return next.run(request).await
    }

    ApiErr(Error::AuthorizationDataNotFound(None)).into_response();
}

pub async fn log_request(req: Request, next: Next) -> Response {
    let (parts, body) = req.into_parts();

    let content_type_header = parts.headers.get(CONTENT_TYPE);
    let content_type = content_type_header.and_then(|value| value.to_str().ok());

    match content_type {
        Some(ct)
            if !ct.starts_with("application/json")
                && !ct.starts_with("application/x-www-form-urlencoded") =>
        {
            info!(method = %parts.method,uri = %parts.uri,headers = ?parts.headers, "--> 接收到非 JSON 或表单请求");

            return next.run(Request::from_parts(parts, body)).await;
        }
        None => {
            info!(method = %parts.method, uri = %parts.uri, headers = ?parts.headers, "--> 接收到未知数据源请求");
            return next.run(Request::from_parts(parts, body)).await;
        }
        _ => {}
    }

    let bytes = get_body_bytes(body).await;

    if let Err(e) = bytes {
        return ApiErr(e).into_response();
    }

    let bytes = bytes.unwrap();

    if let Ok(body) = std::str::from_utf8(&bytes) {
        info!(
            method = %parts.method,
            uri = %parts.uri,
            headers = ?parts.headers,
            ?body,
            "--> 接收到请求"
        );
    }

    next.run(Request::from_parts(parts, Body::from(bytes)))
        .await
}

pub async fn log_response(req: Request, next: Next) -> Response {
    let started_at = Instant::now();

    let response = next.run(req).await;

    let (parts, body) = response.into_parts();

    let content_type_header = parts.headers.get(CONTENT_TYPE);
    let content_type = content_type_header.and_then(|value| value.to_str().ok());

    if let Some(content_type) = content_type
        && !content_type.starts_with("application/json")
        && !content_type.starts_with("application/x-www-form-urlencoded")
    {
        info!(
            elapsed = started_at.elapsed().as_secs_f32(),
            "<-- 请求处理完成"
        );

        return Response::from_parts(parts, body);
    }

    let bytes = get_body_bytes(body).await;

    if let Err(e) = bytes {
        return ApiErr(e).into_response();
    }

    let bytes = bytes.unwrap();

    if let Ok(body) = std::str::from_utf8(&bytes) {
        info!(
            elapsed = started_at.elapsed().as_secs_f32(),
            ?body,
            "<-- 请求处理完成"
        );
    }

    Response::from_parts(parts, Body::from(bytes))
}

async fn get_body_bytes<B>(body: B) -> Result<Bytes>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    match body.collect().await {
        Ok(collected) => Ok(collected.to_bytes()),
        Err(_) => Err(Error::InternalReadBodyFailed(None)),
    }
}
