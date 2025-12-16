use salvo::{handler, Depot, FlowCtrl, Request, Response, Router};
use salvo::prelude::{Json, StatusCode};

pub fn health() -> Router {
    #[handler]
    async fn success() -> &'static str {
        "success"
    }

    Router::with_path("/health").get(success)
}

#[handler]
pub fn catcher(&self, _req: &Request, _depot: &Depot, res: &mut Response, ctrl: &mut FlowCtrl) {
    if StatusCode::NOT_FOUND == res.status_code.unwrap_or(StatusCode::NOT_FOUND) {
        res.render(Json(Response::<String>::new(Some(404), Some("Not Found".to_string()), None)));
        ctrl.skip_rest();
    }
}

pub fn api_v1() -> Router {
    Router::with_path("/api/v1")
        .push(api_v1_account())
        .push(api_v1_totp())
        .push(api_v1_short_url())
}

fn api_v1_account() -> Router {
    Router::with_path("/access-token")
        .push(
            Router::
        )

    // let unauthorized = Router::new().nest(
    //     "/access-token",
    //     Router::new()
    //         .route("/login", post(v1::access_token::login))
    //         .route("/login/refresh", post(v1::access_token::login_refresh)),
    // );
    //
    // let authorized = Router::new()
    //     .nest(
    //         "/access-token",
    //         Router::new().route("/valid", get(v1::access_token::valid)),
    //     )
    //     .nest(
    //         "/users",
    //         Router::new()
    //             .route("/detail", post(v1::users::detail))
    //             .route("/edit/avatar", post(v1::users::edit_avatar))
    //             .route("/edit/nickname", post(v1::users::edit_nickname))
    //             .route("/edit/slogan", post(v1::users::edit_slogan))
    //             .route("/edit/phone", post(v1::users::edit_phone))
    //             .route("/delete", post(v1::users::delete)),
    //     )
    //     .layer(ServiceBuilder::new().layer(middleware::from_fn(authorization)));
    //
    // Router::new().merge(unauthorized).merge(authorized)
}

fn api_v1_totp() -> Router {
    // Router::new()
    //     .nest(
    //         "/totp",
    //         Router::new()
    //             .route("/all", post(v1::totp::all))
    //             .route("/detail", post(v1::totp::detail))
    //             .route("/create", post(v1::totp::create))
    //             .route("/edit/username", post(v1::totp::edit_username))
    //             .route("/edit/issuer", post(v1::totp::edit_issuer))
    //             .route("/delete", post(v1::totp::delete)),
    //     )
    //     .layer(ServiceBuilder::new().layer(middleware::from_fn(authorization)))
}

fn api_v1_short_url() -> Router {
    // let unauthorized = Router::new().nest(
    //     "/short-url",
    //     Router::new()
    //         .route("/detail", post(v1::short_url::detail))
    //         .route("/redirect/{short}", get(v1::short_url::redirect)),
    // );
    //
    // let authorized = Router::new()
    //     .nest(
    //         "/short-url",
    //         Router::new().route("/create", post(v1::short_url::create)),
    //     )
    //     .layer(ServiceBuilder::new().layer(middleware::from_fn(authorization)));
    //
    // Router::new().merge(unauthorized).merge(authorized)
}
