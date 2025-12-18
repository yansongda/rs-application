use crate::response::ApiErr;
use application_database::account::access_token;
use application_kernel::result::Error;
use futures_util::StreamExt;
use salvo::http::header::AUTHORIZATION;
use salvo::http::{mime, Mime};
use salvo::{handler, Depot, FlowCtrl, Request, Response};
use std::time::Instant;
use tracing::{info, Instrument};
use tracing_subscriber::registry::LookupSpan;

#[handler]
pub async fn authorization(
    request: &mut Request,
    depot: &mut Depot,
    response: &mut Response,
    ctrl: &mut FlowCtrl,
) {
    macro_rules! abort {
        ($error:expr) => {{
            response.render(ApiErr($error));
            ctrl.skip_rest();
            return;
        }};
    }

    let auth = match request.headers().get(AUTHORIZATION) {
        Some(h) => match h.to_str() {
            Ok(a) => a,
            Err(_) => abort!(Error::AuthorizationInvalidFormat(None)),
        },
        None => abort!(Error::AuthorizationHeaderMissing(None)),
    };

    let token = auth.strip_prefix("Bearer ").unwrap_or(auth);
    let access_token = match access_token::fetch(token).await {
        Ok(t) if !t.is_expired() => t,
        Ok(_) => abort!(Error::AuthorizationAccessTokenExpired(None)),
        Err(_) => abort!(Error::AuthorizationAccessTokenInvalid(None)),
    };

    depot.inject(access_token);

    ctrl.call_next(request, depot, response).await;
}

#[handler]
pub async fn request_logger(
    request: &mut Request,
    depot: &mut Depot,
    response: &mut Response,
    ctrl: &mut FlowCtrl,
) {
    let request_id = request
        .headers()
        .get("x-request-id")
        .map_or_else(|| "unknown", |v| v.to_str().unwrap_or("unknown"));

    let span = tracing::info_span!("root", request_id);

    span.with_subscriber(|(id, dispatch)| {
        if let Some(sub) = dispatch.downcast_ref::<tracing_subscriber::Registry>()
            && let Some(span_ref) = sub.span(id)
        {
            span_ref
                .extensions_mut()
                .insert(application_kernel::logger::TracingId(
                    request_id.to_string(),
                ));
        }
    });

    let (message, body) = match request.content_type() {
        Some(ct) if is_loggable_mime(&ct) => {
            let body = request
                .parse_body::<&str>()
                .await
                .unwrap_or("未解析出请求 body");
            ("--> 接收到请求", body.to_string())
        }
        Some(_) => ("--> 接收到非 JSON 或表单请求", String::new()),
        None => ("--> 接收到未知数据源请求", String::new()),
    };

    async move {
        info!(
            message,
            method = %request.method(),
            uri = %request.uri(),
            headers = ?request.headers(),
            body,
        );

        let now = Instant::now();

        ctrl.call_next(request, depot, response).await;

        let elapsed = now.elapsed().as_secs_f32();

        let body = match response.content_type() {
            Some(ct) if is_loggable_mime(&ct) => {
                let mut body = response.take_body();
                let mut bytes = Vec::new();

                while let Some(Ok(chunk)) = body.next().await {
                    if let Ok(data) = chunk.into_data() {
                        bytes.extend_from_slice(&data);
                    }
                }

                let res_body = String::from_utf8_lossy(&bytes).to_string();

                response.body(res_body.to_owned());

                res_body
            }
            _ => String::new(),
        };

        info!(message = "<-- 请求处理完成", elapsed, body);
    }
    .instrument(span)
    .await
}

fn is_loggable_mime(ct: &Mime) -> bool {
    ct.to_string()
        .contains(mime::APPLICATION_JSON.to_string().as_str())
        || ct
            .to_string()
            .contains(mime::APPLICATION_WWW_FORM_URLENCODED.to_string().as_str())
}
