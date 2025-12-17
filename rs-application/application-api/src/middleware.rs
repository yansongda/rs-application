use crate::response::ApiErr;
use application_database::account::access_token;
use application_kernel::result::Error;
use salvo::{Depot, FlowCtrl, Request, Response, handler};

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

    let auth = match request.headers().get("Authorization") {
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

// pub async fn log_request(req: Request, next: Next) -> Response {
//     let (parts, body) = req.into_parts();

//     let content_type_header = parts.headers.get(CONTENT_TYPE);
//     let content_type = content_type_header.and_then(|value| value.to_str().ok());

//     match content_type {
//         Some(ct)
//             if !ct.starts_with("application/json")
//                 && !ct.starts_with("application/x-www-form-urlencoded") =>
//         {
//             info!(method = %parts.method,uri = %parts.uri,headers = ?parts.headers, "--> 接收到非 JSON 或表单请求");

//             return next.run(Request::from_parts(parts, body)).await;
//         }
//         None => {
//             info!(method = %parts.method, uri = %parts.uri, headers = ?parts.headers, "--> 接收到未知数据源请求");
//             return next.run(Request::from_parts(parts, body)).await;
//         }
//         _ => {}
//     }

//     let bytes = get_body_bytes(body).await;

//     if let Err(e) = bytes {
//         return ApiErr(e).into_response();
//     }

//     let bytes = bytes.unwrap();

//     if let Ok(body) = std::str::from_utf8(&bytes) {
//         info!(
//             method = %parts.method,
//             uri = %parts.uri,
//             headers = ?parts.headers,
//             ?body,
//             "--> 接收到请求"
//         );
//     }

//     next.run(Request::from_parts(parts, Body::from(bytes)))
//         .await
// }

// pub async fn log_response(req: Request, next: Next) -> Response {
//     let started_at = Instant::now();

//     let response = next.run(req).await;

//     let (parts, body) = response.into_parts();

//     let content_type_header = parts.headers.get(CONTENT_TYPE);
//     let content_type = content_type_header.and_then(|value| value.to_str().ok());

//     if let Some(content_type) = content_type
//         && !content_type.starts_with("application/json")
//         && !content_type.starts_with("application/x-www-form-urlencoded")
//     {
//         info!(
//             elapsed = started_at.elapsed().as_secs_f32(),
//             "<-- 请求处理完成"
//         );

//         return Response::from_parts(parts, body);
//     }

//     let bytes = get_body_bytes(body).await;

//     if let Err(e) = bytes {
//         return ApiErr(e).into_response();
//     }

//     let bytes = bytes.unwrap();

//     if let Ok(body) = std::str::from_utf8(&bytes) {
//         info!(
//             elapsed = started_at.elapsed().as_secs_f32(),
//             ?body,
//             "<-- 请求处理完成"
//         );
//     }

//     Response::from_parts(parts, Body::from(bytes))
// }

// async fn get_body_bytes<B>(body: B) -> Result<Bytes>
// where
//     B: axum::body::HttpBody<Data = Bytes>,
//     B::Error: std::fmt::Display,
// {
//     match body.collect().await {
//         Ok(collected) => Ok(collected.to_bytes()),
//         Err(_) => Err(Error::InternalReadBodyFailed(None)),
//     }
// }
