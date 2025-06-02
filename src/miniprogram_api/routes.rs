use axum::routing::{get, post};
use axum::{Router, middleware};
use tower::ServiceBuilder;

use crate::miniprogram_api::middleware::authorization;
use crate::miniprogram_api::v1;

pub fn api_v1_miniprogram() -> Router {
    let unauthorized = Router::new()
        .nest(
            "/access-token",
            Router::new().route("/login", post(v1::access_token::login)),
        )
        .nest(
            "/short-url",
            Router::new()
                .route("/detail", post(v1::short_url::detail))
                .route("/redirect/{short}", get(v1::short_url::redirect)),
        );

    let authorized = Router::new()
        .nest(
            "/access-token",
            Router::new().route("/valid", get(v1::access_token::valid)),
        )
        .nest(
            "/users",
            Router::new()
                .route("/detail", post(v1::users::detail))
                .route("/edit/nickname", post(v1::users::edit_nickname))
                .route("/edit/slogan", post(v1::users::edit_slogan))
                .route("/edit/phone", post(v1::users::edit_phone))
                .route("/edit", post(v1::users::edit)),
        )
        .nest(
            "/totp",
            Router::new()
                .route("/all", post(v1::totp::all))
                .route("/detail", post(v1::totp::detail))
                .route("/create", post(v1::totp::create))
                .route("/edit", post(v1::totp::edit))
                .route("/delete", post(v1::totp::delete)),
        )
        .nest(
            "/short-url",
            Router::new().route("/create", post(v1::short_url::create)),
        )
        .layer(ServiceBuilder::new().layer(middleware::from_fn(authorization)));

    authorized.merge(unauthorized)
}
