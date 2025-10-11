use reqwest::{Client, Method, Request, RequestBuilder, Url};

use crate::http;
use application_kernel::result::{Error, Result};
use serde::Deserialize;

static THIRD_HTTP_REQUEST_MESSAGE: &str = "第三方错误: 华为 API 请求出错，请联系管理员";

static THIRD_HTTP_RESPONSE_MESSAGE: &str = "第三方错误: 华为 API 响应接收出错，请联系管理员";

static THIRD_HTTP_RESPONSE_PARSE_MESSAGE: &str = "第三方错误: 华为 API 响应解析出错，请联系管理员";

static THIRD_HTTP_RESPONSE_RESULT_MESSAGE: &str = "第三方错误: 华为 API 响应结果出错，请联系管理员";

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct TokenResponse {
    pub token_type: String,
    pub access_token: String,
    pub scope: String,
    pub expires_in: i64,
    pub refresh_token: String,
    pub id_token: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct TokenResponseError {
    pub error: i64,
    pub error_description: String,
}

pub async fn token(code: &str, app_id: &str, app_secret: &str) -> Result<TokenResponse> {
    let form = [
        ("grant_type", "authorization_code"),
        ("client_id", app_id),
        ("client_secret", app_secret),
        ("code", code),
    ];

    let builder = RequestBuilder::from_parts(
        Client::new(),
        Request::new(
            Method::POST,
            Url::parse("https://oauth-login.cloud.huawei.com/oauth2/v3/token").unwrap(),
        ),
    )
    .form(&form);

    let response = http::request::<TokenResponse, TokenResponseError>(builder.build().unwrap())
        .await
        .map_err(|e| map_request_err(e))?;

    if !response.inner.is_success() {
        return Err(Error::ThirdHttpResponseResult(Some(
            THIRD_HTTP_RESPONSE_RESULT_MESSAGE,
        )));
    }

    Ok(response.inner.into_success().unwrap())
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct TokenInfoResponse {
    pub client_id: String,
    pub expires_in: i64,
    pub union_id: String,
    pub project_id: String,
    #[serde(rename = "type")]
    pub r#type: i64,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct TokenInfoResponseError {
    pub error: String,
}

pub async fn token_info(access_token: &str) -> Result<TokenInfoResponse> {
    let form = [("access_token", access_token)];

    let builder = RequestBuilder::from_parts(
        Client::new(),
        Request::new(
            Method::POST,
            Url::parse("https://oauth-api.cloud.huawei.com/rest.php?nsp_fmt=JSON&nsp_svc=huawei.oauth2.user.getTokenInfo").unwrap()
        ),
    )
        .form(&form);

    let response =
        http::request::<TokenInfoResponse, TokenInfoResponseError>(builder.build().unwrap())
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
