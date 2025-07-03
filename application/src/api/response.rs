use axum::body::Body;
use axum::extract::rejection::JsonRejection;
use axum::response::IntoResponse;
use serde::Serialize;
use tracing::info;

use application_kernel::result::{Error, Response, Result};

pub type Resp<D> = Result<Response<D>>;

impl<D: Serialize> Response<D> {
    fn to_http_response(&self) -> axum::http::Response<Body> {
        let body = serde_json::to_string(self).unwrap();

        axum::response::Response::builder()
            .status(200)
            .header("Content-Type", "application/json")
            .body(Body::from(body))
            .unwrap()
    }
}

impl<D: Serialize> IntoResponse for Response<D> {
    fn into_response(self) -> axum::response::Response {
        self.to_http_response().into_response()
    }
}

impl Error {
    fn to_http_response(self) -> axum::http::Response<Body> {
        Response::<String>::error(self).to_http_response()
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        self.to_http_response().into_response()
    }
}

impl From<JsonRejection> for Error {
    fn from(r: JsonRejection) -> Self {
        info!("解析 Json 请求失败: {:?}", r);

        Error::ParamsJsonInvalid(None)
    }
}
