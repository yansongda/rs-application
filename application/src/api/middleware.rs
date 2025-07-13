use axum::body::{Body, Bytes};
use application_database::account::access_token;

use axum::extract::Request;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

use crate::api::response::ApiErr;
use application_kernel::result::{Error, Result};

pub async fn authorization(mut request: Request, next: Next) -> Response {
    let authorization = request.headers().get("Authorization");

    if authorization.is_none() {
        return ApiErr(Error::AuthorizationHeaderMissing(None)).into_response();
    }

    let auth = authorization.unwrap().to_str();

    if auth.is_err() {
        return ApiErr(Error::AuthorizationInvalidFormat(None)).into_response();
    }

    let access_token: Result<access_token::AccessToken> =
        access_token::fetch(auth.unwrap().replace("Bearer ", "").as_str())
            .await
            .map_err(|_| Error::AuthorizationDataNotFound(None));

    if let Err(e) = access_token {
        return ApiErr(e).into_response();
    }

    request.extensions_mut().insert(access_token.unwrap());

    next.run(request).await
}

async fn print_request_response(
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let (parts, body) = req.into_parts();
    let bytes = buffer_and_print("request", body).await?;
    let req = Request::from_parts(parts, Body::from(bytes));

    let res = next.run(req).await;

    let (parts, body) = res.into_parts();
    let bytes = buffer_and_print("response", body).await?;
    let res = Response::from_parts(parts, Body::from(bytes));

    Ok(res)
}

async fn buffer_and_print<B>(direction: &str, body: B) -> Result<Bytes, (StatusCode, String)>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes = match body.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(err) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("failed to read {direction} body: {err}"),
            ));
        }
    };

    if let Ok(body) = std::str::from_utf8(&bytes) {
        tracing::debug!("{direction} body = {body:?}");
    }

    Ok(bytes)
}
