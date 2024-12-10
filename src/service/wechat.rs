use reqwest::{Method, Request, Url};
use tracing::error;

use crate::config::G_CONFIG;
use crate::model::result::{Error, Result};
use crate::model::wechat::LoginResponse;
use crate::util::http;

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
        Error::Http(message) => Error::HttpWechat(message),
        Error::HttpResponse(message) => Error::HttpWechatResponse(message),
        _ => Error::Http(None),
    })?;

    let result: LoginResponse = serde_json::from_str(response.body.as_str())
        .map_err(|_| Error::HttpWechatResponseParse(None))?;

    if result.errcode.is_some() {
        error!("微信 API 结果出错: {:?}", result);

        return Err(Error::HttpWechatResponse(None));
    }

    Ok(result)
}
