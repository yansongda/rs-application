use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::LazyLock;
use std::time::Duration;

use reqwest::{Client, Request};
use serde::Deserialize;
use tracing::{info, warn};

use application_kernel::result::{Error, Result};

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct HttpResponse<S, E> {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub duration: f32,
    pub inner: ResponseVariant<S, E>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ResponseVariant<S, E> {
    Success(S),
    Error(E),
}

impl<S, E> ResponseVariant<S, E> {
    pub fn parse(body: &str) -> Result<Self>
    where
        S: Debug + for<'de> Deserialize<'de>,
        E: Debug + for<'de> Deserialize<'de>,
    {
        serde_json::from_str::<ResponseVariant<S, E>>(body)
            .map_err(|_| Error::ThirdHttpResponseParse(None))
    }

    pub fn is_success(&self) -> bool {
        matches!(self, ResponseVariant::Success(_))
    }

    pub fn into_success(self) -> Option<S> {
        match self {
            ResponseVariant::Success(success) => Some(success),
            ResponseVariant::Error(_) => None,
        }
    }

    pub fn into_error(self) -> Option<E> {
        match self {
            ResponseVariant::Error(error) => Some(error),
            ResponseVariant::Success(_) => None,
        }
    }
}

static G_CLIENT: LazyLock<Client> = LazyLock::new(|| {
    Client::builder()
        .user_agent("yansongda/rs-application")
        .connect_timeout(Duration::from_secs(1))
        .timeout(Duration::from_secs(3))
        .build()
        .unwrap()
});

pub async fn request<S, E>(request: Request) -> Result<HttpResponse<S, E>>
where
    S: Debug + for<'de> Deserialize<'de>,
    E: Debug + for<'de> Deserialize<'de>,
{
    info!("请求第三方服务接口 {:?}", request);

    let started_at = std::time::Instant::now();
    let response = G_CLIENT.execute(request).await.map_err(|e| {
        warn!("请求第三方服务接口失败 {:?}", e);
        Error::ThirdHttpRequest(None)
    })?;

    let status = response.status().as_u16();
    let headers = response
        .headers()
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
        .collect::<HashMap<String, String>>();

    let body = response.text().await.map_err(|e| {
        warn!("接收第三方服务接口响应失败 {:?}", e);
        Error::ThirdHttpResponse(None)
    })?;

    info!("请求第三方服务接口原始结果 {:?}", &body);

    let result = HttpResponse {
        status,
        headers,
        body: body.clone(),
        duration: started_at.elapsed().as_secs_f32(),
        inner: serde_json::from_str::<ResponseVariant<S, E>>(&body)
            .map_err(|_| Error::ThirdHttpResponseParse(None))?,
    };

    info!("请求第三方服务接口结果 {:?}", result);

    if !result.inner.is_success() {
        return Err(Error::ThirdHttpResponseResult(None));
    }

    Ok(result)
}

pub fn map_request_err(e: Error, platform: &str) -> Error {
    match e {
        Error::ThirdHttpRequest(_) => Error::ThirdHttpRequest(Some(format!(
            "第三方错误: {} API 请求出错，请联系管理员",
            platform
        ))),
        Error::ThirdHttpResponse(_) => Error::ThirdHttpResponse(Some(format!(
            "第三方错误: {} API 响应接收出错，请联系管理员",
            platform
        ))),
        Error::ThirdHttpResponseParse(_) => Error::ThirdHttpResponseParse(Some(format!(
            "第三方错误: {} API 响应解析出错，请联系管理员",
            platform
        ))),
        Error::ThirdHttpResponseResult(_) => Error::ThirdHttpResponseResult(Some(format!(
            "第三方错误: {} API 响应业务结果出错，请联系管理员",
            platform
        ))),
        _ => Error::ThirdHttpRequest(None),
    }
}
