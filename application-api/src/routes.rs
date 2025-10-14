use axum::routing::{get, post};
use axum::{Router, middleware};
use tower::ServiceBuilder;

use crate::middleware::authorization;
use crate::v1;

pub fn api_v1() -> Router {
    Router::new().merge(api_v1_account()).merge(api_v1_tool())
}

fn api_v1_account() -> Router {
    let unauthorized = Router::new().nest(
        "/access-token",
        Router::new()
            .route("/login", post(v1::access_token::login))
            .route("/login/refresh", post(v1::access_token::login_refresh)),
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
                .route("/edit/avatar", post(v1::users::edit_avatar))
                .route("/edit/nickname", post(v1::users::edit_nickname))
                .route("/edit/slogan", post(v1::users::edit_slogan))
                .route("/edit/phone", post(v1::users::edit_phone)),
        )
        .layer(ServiceBuilder::new().layer(middleware::from_fn(authorization)));

    Router::new().merge(unauthorized).merge(authorized)
}

fn api_v1_tool() -> Router {
    let unauthorized = Router::new().nest(
        "/short-url",
        Router::new()
            .route("/detail", post(v1::short_url::detail))
            .route("/redirect/{short}", get(v1::short_url::redirect)),
    );

    let authorized = Router::new()
        .nest(
            "/totp",
            Router::new()
                .route("/all", post(v1::totp::all))
                .route("/detail", post(v1::totp::detail))
                .route("/create", post(v1::totp::create))
                .route("/edit/username", post(v1::totp::edit_username))
                .route("/edit/issuer", post(v1::totp::edit_issuer))
                .route("/delete", post(v1::totp::delete)),
        )
        .nest(
            "/short-url",
            Router::new().route("/create", post(v1::short_url::create)),
        )
        .layer(ServiceBuilder::new().layer(middleware::from_fn(authorization)));

    Router::new().merge(unauthorized).merge(authorized)
}
