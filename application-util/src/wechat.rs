use reqwest::{Method, Request, Url};

use crate::http;
use crate::http::map_request_err;
use application_kernel::result::Result;
use serde::Deserialize;

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
    .map_err(|e| map_request_err(e, "微信"))?;

    Ok(response.inner.into_success().unwrap())
}
