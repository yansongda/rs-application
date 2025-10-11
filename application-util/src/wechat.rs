use reqwest::{Method, Request, Url};

use crate::http;
use application_kernel::result::{Error, Result};
use serde::Deserialize;

static THIRD_HTTP_REQUEST_MESSAGE: &str = "第三方错误: 华为 API 请求出错，请联系管理员";

static THIRD_HTTP_RESPONSE_MESSAGE: &str = "第三方错误: 微信 API 响应接收出错，请联系管理员";

static THIRD_HTTP_RESPONSE_PARSE_MESSAGE: &str = "第三方错误: 微信 API 响应解析出错，请联系管理员";

static THIRD_HTTP_RESPONSE_RESULT_MESSAGE: &str = "第三方错误: 微信 API 响应结果出错，请联系管理员";

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct LoginResponse {
    pub session_key: String,
    pub unionid: String,
    pub openid: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct LoginResponseError {
    pub errmsg: String,
    pub errcode: i32,
}

pub async fn login(code: &str, app_id: &str, app_secret: &str) -> Result<LoginResponse> {
    let query = [
        ("appid", app_id),
        ("secret", app_secret),
        ("js_code", code),
        ("grant_type", "authorization_code"),
    ];

    let response = http::request::<LoginResponse, LoginResponseError>(Request::new(
        Method::GET,
        Url::parse_with_params("https://api.weixin.qq.com/sns/jscode2session", query).unwrap(),
    ))
    .await
    .map_err(|e| map_request_err(e))?;

    if !response.inner.is_success() {
        return Err(Error::ThirdHttpResponseResult(Some(
            THIRD_HTTP_RESPONSE_RESULT_MESSAGE,
        )));
    }

    Ok(response.inner.into_success().unwrap())
}

fn map_request_err(e: Error) -> Error {
    match e {
        Error::ThirdHttpRequest(_) => Error::ThirdHttpRequest(Some(THIRD_HTTP_REQUEST_MESSAGE)),
        Error::ThirdHttpResponse(_) => Error::ThirdHttpResponse(Some(THIRD_HTTP_RESPONSE_MESSAGE)),
        Error::ThirdHttpResponseParse(_) => {
            Error::ThirdHttpResponseParse(Some(THIRD_HTTP_RESPONSE_PARSE_MESSAGE))
        }
        _ => Error::ThirdHttpRequest(None),
    }
}
