use reqwest::{Method, Request, Url};
use tracing::error;

use crate::http;
use application_kernel::result::{Error, Result};
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct TokenResponse {
    pub token_type: String,
    pub access_token: String,
    pub scope: String,
    pub expires_in: i64,
    pub refresh_token: String,
    pub id_token: String
}

pub async fn token(code: &str, app_id: &str, app_secret: &str) -> Result<TokenResponse> {
    let query = [
        ("appid", app_id),
        ("secret", app_secret),
        ("js_code", code),
        ("grant_type", "authorization_code"),
    ];

    let response = http::request(Request::new(
        Method::GET,
        Url::parse_with_params("https://api.weixin.qq.com/sns/jscode2session", query).unwrap(),
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
