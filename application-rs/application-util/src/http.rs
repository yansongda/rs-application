use std::collections::HashMap;
use std::sync::LazyLock;
use std::time::Duration;

use reqwest::{Client, Request};
use serde::Deserialize;
use tracing::{info, warn};

use application_kernel::result::{Error, Result};

#[derive(Debug)]
pub struct HttpResponse<T> {
    pub status: u16,
    pub duration: f32,
    pub inner: T,
}

static G_CLIENT: LazyLock<Client> = LazyLock::new(|| {
    Client::builder()
        .user_agent("yansongda/application-rs")
        .connect_timeout(Duration::from_secs(1))
        .timeout(Duration::from_secs(3))
        .build()
        .expect("HTTP 客户端初始化失败")
});

pub async fn request<T>(req: Request) -> Result<HttpResponse<T>>
where
    T: for<'de> Deserialize<'de>,
{
    info!("请求第三方服务接口 {:?}", req);

    let started_at = std::time::Instant::now();
    let response = G_CLIENT.execute(req).await.map_err(|e| {
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

    let inner =
        serde_json::from_str::<T>(&body).map_err(|_| Error::ThirdHttpResponseParse(None))?;

    Ok(HttpResponse {
        status,
        duration: started_at.elapsed().as_secs_f32(),
        inner,
    })
}
