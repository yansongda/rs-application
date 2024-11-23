use std::collections::HashMap;
use std::sync::OnceLock;
use std::time::Duration;

use reqwest::{Client, Request};
use tracing::{info, warn};

use crate::model::http::HttpResponse;
use crate::model::result::{Error, Result};

static G_CLIENT: OnceLock<Client> = OnceLock::new();

pub async fn request(request: Request) -> Result<HttpResponse> {
    let client = G_CLIENT.get_or_init(|| {
        Client::builder()
            .user_agent("yansongda/rs-application")
            .connect_timeout(Duration::from_secs(1))
            .timeout(Duration::from_secs(3))
            .build()
            .unwrap()
    });

    info!("请求第三方服务接口 {:?}", request);

    let started_at = std::time::Instant::now();
    let response = client.execute(request).await.map_err(|e| {
        warn!("请求第三方服务接口失败 {:?}", e);

        Error::Http(None)
    })?;

    let result = HttpResponse {
        status: response.status().as_u16(),
        headers: response
            .headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap().to_string()))
            .collect::<HashMap<String, String>>(),
        body: response
            .text()
            .await
            .map_err(|_| Error::HttpResponse(None))?,
        duration: started_at.elapsed().as_secs_f32(),
    };

    info!("请求第三方服务接口结果 {:?}", result);

    Ok(result)
}
