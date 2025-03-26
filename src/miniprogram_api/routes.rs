use axum::routing::{get, post};
use axum::{Router, middleware};
use tower::ServiceBuilder;

use crate::miniprogram_api::middleware::authorization;
use crate::miniprogram_api::v1;

pub fn api_v1_miniprogram() -> Router {
    let unauthorized = Router::new()
        .route("/access-token/login", post(v1::access_token::login))
        .route("/short-url/detail", post(v1::short_url::detail))
        .route("/short-url/redirect/{short}", get(v1::short_url::redirect));

    let authorized = Router::new()
        .nest(
            "/users",
            Router::new()
                .route("/detail", post(v1::users::detail))
                .route("/update", post(v1::users::update)),
        )
        .nest(
            "/totp",
            Router::new()
                .route("/all", post(v1::totp::all))
                .route("/detail", post(v1::totp::detail))
                .route("/create", post(v1::totp::create))
                .route("/update", post(v1::totp::update))
                .route("/delete", post(v1::totp::delete)),
        )
        .nest(
            "/short-url",
            Router::new().route("/create", post(v1::short_url::create)),
        )
        .layer(ServiceBuilder::new().layer(middleware::from_fn(authorization)));

    authorized.merge(unauthorized)
}
