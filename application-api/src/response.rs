use axum::body::Body;
use axum::extract::rejection::JsonRejection;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use tracing::info;

use application_kernel::result::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<D: Serialize> {
    pub code: u16,
    pub message: String,
    pub data: Option<D>,
}

impl<D: Serialize> Response<D> {
    pub fn new(code: Option<u16>, message: Option<String>, data: Option<D>) -> Self {
        Response {
            code: code.unwrap_or(0),
            message: message.unwrap_or_else(|| "success".to_string()),
            data,
        }
    }

    pub fn success(data: D) -> Self {
        Response::new(None, None, Some(data))
    }

    pub fn error(err: ApiErr) -> Self {
        let (code, message) = err.0.get_code_message();

        Response::new(Some(code), Some(message.to_string()), None)
    }

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

pub type Result<D> = std::result::Result<D, ApiErr>;
pub type Resp<D> = Result<Response<D>>;

pub struct ApiErr(pub Error);

impl IntoResponse for ApiErr {
    fn into_response(self) -> axum::response::Response {
        Response::<String>::error(self)
            .to_http_response()
            .into_response()
    }
}

impl From<Error> for ApiErr {
    fn from(r: Error) -> Self {
        ApiErr(r)
    }
}

impl From<JsonRejection> for ApiErr {
    fn from(r: JsonRejection) -> Self {
        info!("解析 Json 请求失败: {:?}", r);

        ApiErr::from(Error::ParamsJsonInvalid(None))
    }
}
