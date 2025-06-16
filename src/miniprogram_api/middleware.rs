use crate::{model, repository};

use axum::extract::Request;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

use crate::model::result::{Error, Result};

pub async fn authorization(mut request: Request, next: Next) -> Response {
    let authorization = request.headers().get("Authorization");

    if authorization.is_none() {
        return Error::AuthorizationMiniprogramHeaderMissing(None).into_response();
    }

    let auth = authorization.unwrap().to_str();

    if auth.is_err() {
        return Error::AuthorizationMiniprogramInvalidFormat(None).into_response();
    }

    let access_token: Result<model::miniprogram::access_token::AccessToken> =
        repository::miniprogram::access_token::fetch(auth.unwrap().replace("Bearer ", "").as_str())
            .await
            .map_err(|_| Error::AuthorizationMiniprogramDataNotFound(None));

    if let Err(e) = access_token {
        return e.into_response();
    }

    request.extensions_mut().insert(access_token.unwrap());

    next.run(request).await
}
