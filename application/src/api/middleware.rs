use application_database::account::access_token;

use axum::extract::Request;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

use application_kernel::result::{Error, Result};

pub async fn authorization(mut request: Request, next: Next) -> Response {
    let authorization = request.headers().get("Authorization");

    if authorization.is_none() {
        return Error::AuthorizationHeaderMissing(None).into_response();
    }

    let auth = authorization.unwrap().to_str();

    if auth.is_err() {
        return Error::AuthorizationInvalidFormat(None).into_response();
    }

    let access_token: Result<access_token::AccessToken> =
        access_token::fetch(auth.unwrap().replace("Bearer ", "").as_str())
            .await
            .map_err(|_| Error::AuthorizationDataNotFound(None));

    if let Err(e) = access_token {
        return e.into_response();
    }

    request.extensions_mut().insert(access_token.unwrap());

    next.run(request).await
}
