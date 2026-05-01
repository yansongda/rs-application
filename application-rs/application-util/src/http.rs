use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::LazyLock;
use std::time::Duration;

use reqwest::{Client, Request};
use serde::{Deserialize, Deserializer, de, de::DeserializeOwned};
use serde_json::Value;
use tracing::{info, warn};

use application_kernel::result::{Error, Result};

#[allow(dead_code)]
#[derive(Debug)]
pub struct HttpResponse<S, E> {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub duration: f32,
    pub inner: ResponseVariant<S, E>,
}

impl<'de, S, E> Deserialize<'de> for HttpResponse<S, E>
where
    S: Debug + DeserializeOwned,
    E: Debug + DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawHttpResponse<S, E> {
            status: u16,
            headers: HashMap<String, String>,
            body: String,
            duration: f32,
            inner: Value,
            #[serde(skip)]
            _marker: std::marker::PhantomData<(S, E)>,
        }

        let raw = RawHttpResponse::<S, E>::deserialize(deserializer)?;
        let inner = serde_json::from_value::<ResponseVariant<S, E>>(raw.inner)
            .map_err(de::Error::custom)?;

        Ok(Self {
            status: raw.status,
            headers: raw.headers,
            body: raw.body,
            duration: raw.duration,
            inner,
        })
    }
}

#[derive(Debug)]
pub enum ResponseVariant<S, E> {
    Success(S),
    Error(E),
}

impl<S, E> ResponseVariant<S, E> {
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

impl<'de, S, E> Deserialize<'de> for ResponseVariant<S, E>
where
    S: Debug + for<'d> Deserialize<'d>,
    E: Debug + for<'d> Deserialize<'d>,
{
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;

        // 判别逻辑 1：errcode == 0 → Success（微信）
        if let Some(errcode) = value.get("errcode").and_then(|errcode| errcode.as_u64()) {
            return if errcode == 0 {
                serde_json::from_value::<S>(value)
                    .map(ResponseVariant::Success)
                    .map_err(de::Error::custom)
            } else {
                serde_json::from_value::<E>(value)
                    .map(ResponseVariant::Error)
                    .map_err(de::Error::custom)
            };
        }

        // 判别逻辑 2：error 存在且非空 → Error（华为）
        if let Some(error) = value.get("error").and_then(|error| error.as_str())
            && !error.is_empty()
        {
            return serde_json::from_value::<E>(value)
                .map(ResponseVariant::Error)
                .map_err(de::Error::custom);
        }

        // 兜底：先尝试 Success，失败则尝试 Error
        serde_json::from_value::<S>(value.clone())
            .map(ResponseVariant::Success)
            .or_else(|_| serde_json::from_value::<E>(value).map(ResponseVariant::Error))
            .map_err(de::Error::custom)
    }
}

static G_CLIENT: LazyLock<Client> = LazyLock::new(|| {
    Client::builder()
        .user_agent("yansongda/application-rs")
        .connect_timeout(Duration::from_secs(1))
        .timeout(Duration::from_secs(3))
        .build()
        .expect("HTTP 客户端初始化失败")
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

    info!(
        "请求第三方服务接口结果： headers: {:?}, body: {:?}",
        &headers, &body
    );

    let inner = serde_json::from_str::<ResponseVariant<S, E>>(&body)
        .map_err(|_| Error::ThirdHttpResponseParse(None))?;

    if !inner.is_success() {
        return Err(Error::ThirdHttpResponseResult(None));
    }

    Ok(HttpResponse {
        status,
        headers,
        body,
        duration: started_at.elapsed().as_secs_f32(),
        inner,
    })
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
