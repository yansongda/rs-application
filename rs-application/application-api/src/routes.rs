use crate::middleware::authorization;
use crate::v1;
use salvo::prelude::StatusCode;
use salvo::{Depot, FlowCtrl, Request, Response, Router, handler};

#[handler]
pub fn catcher(_req: &Request, _depot: &Depot, res: &mut Response, ctrl: &mut FlowCtrl) {
    let (code, msg) = match res.status_code {
        Some(StatusCode::NOT_FOUND) => (404, "Not Found"),
        Some(StatusCode::METHOD_NOT_ALLOWED) => (405, "Method Not Allowed"),
        _ => return,
    };

    // Use Response's Scribe implementation to ensure request_id is injected
    res.render(crate::response::Response::<String>::new(
        Some(code),
        Some(msg.to_string()),
        None,
    ));

    ctrl.skip_rest();
}

pub fn health() -> Router {
    #[handler]
    async fn success() -> crate::response::Response<&'static str> {
        crate::response::Response::success("success")
    }

    Router::with_path("/health").get(success)
}

pub fn api_v1() -> Router {
    Router::with_path("/api/v1")
        .push(api_v1_access_token())
        .push(api_v1_users())
        .push(api_v1_totp())
        .push(api_v1_short_url())
}

fn api_v1_access_token() -> Router {
    Router::with_path("/access-token")
        .push(
            Router::with_path("/login")
                .post(v1::access_token::login)
                .push(Router::with_path("/refresh").post(v1::access_token::login_refresh)),
        )
        .push(
            Router::with_path("/valid")
                .hoop(authorization)
                .get(v1::access_token::valid),
        )
}

fn api_v1_users() -> Router {
    Router::with_path("/users")
        .hoop(authorization)
        .push(Router::with_path("/detail").post(v1::users::detail))
        .push(
            Router::with_path("/edit")
                .push(Router::with_path("/avatar").post(v1::users::edit_avatar))
                .push(Router::with_path("/nickname").post(v1::users::edit_nickname))
                .push(Router::with_path("/slogan").post(v1::users::edit_slogan))
                .push(Router::with_path("/phone").post(v1::users::edit_phone)),
        )
        .push(Router::with_path("/delete").post(v1::users::delete))
}

fn api_v1_totp() -> Router {
    Router::with_path("/totp")
        .hoop(authorization)
        .push(Router::with_path("/all").post(v1::totp::all))
        .push(Router::with_path("/detail").post(v1::totp::detail))
        .push(Router::with_path("/create").post(v1::totp::create))
        .push(
            Router::with_path("/edit")
                .push(Router::with_path("/username").post(v1::totp::edit_username))
                .push(Router::with_path("/issuer").post(v1::totp::edit_issuer)),
        )
        .push(Router::with_path("/delete").post(v1::totp::delete))
}

fn api_v1_short_url() -> Router {
    Router::with_path("/short-url")
        .push(Router::with_path("/detail").post(v1::short_url::detail))
        .push(Router::with_path("/redirect/{short}").get(v1::short_url::redirect))
        .push(
            Router::with_path("/create")
                .hoop(authorization)
                .post(v1::short_url::create),
        )
}
