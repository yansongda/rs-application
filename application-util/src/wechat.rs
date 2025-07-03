use reqwest::{Method, Request, Url};
use tracing::error;

use crate::http;
use application_kernel::config::G_CONFIG;
use application_kernel::result::{Error, Result};
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct LoginResponse {
    pub session_key: Option<String>,
    pub unionid: Option<String>,
    pub errmsg: Option<String>,
    pub openid: Option<String>,
    pub errcode: Option<i32>,
}

pub async fn login(code: &str) -> Result<LoginResponse> {
    let url = format!("{}/sns/jscode2session", G_CONFIG.wechat.url.as_str());

    let query = [
        ("appid", G_CONFIG.wechat.app_id.as_str()),
        ("secret", G_CONFIG.wechat.app_secret.as_str()),
        ("js_code", code),
        ("grant_type", "authorization_code"),
    ];

    let response = http::request(Request::new(
        Method::GET,
        Url::parse_with_params(url.as_str(), query).unwrap(),
    ))
    .await
    .map_err(|e| match e {
        Error::ThirdHttpRequest(message) => Error::ThirdHttpWechatRequest(message),
        Error::ThirdHttpResponse(message) => Error::ThirdHttpWechatResponse(message),
        _ => Error::ThirdHttpRequest(None),
    })?;

    let result: LoginResponse = serde_json::from_str(response.body.as_str())
        .map_err(|_| Error::ThirdHttpWechatResponseParse(None))?;

    if result.errcode.is_some() {
        error!("微信 API 结果出错: {:?}", result);

        return Err(Error::ThirdHttpWechatResponseCode(None));
    }

    Ok(result)
}
