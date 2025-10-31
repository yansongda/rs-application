use crate::response::Response;
use application_kernel::config::G_CONFIG;
use axum::Router;
use axum::http::Request;
use axum::routing::get;
use std::fmt::Debug;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::request_id::{
    MakeRequestUuid, PropagateRequestIdLayer, RequestId, SetRequestIdLayer,
};
use tower_http::trace::{MakeSpan, OnFailure, TraceLayer};
use tracing::{Span, error, info_span};
use tracing_subscriber::registry::LookupSpan;

mod extract;
mod middleware;
mod request;
mod response;
mod routes;
mod service;
mod v1;

pub struct App {
    listen: SocketAddr,
    router: Router,
}

impl App {
    pub fn init() -> Self {
        App {
            listen: App::listen(),
            router: App::router(),
        }
    }

    pub fn get_listen(&self) -> &SocketAddr {
        &self.listen
    }

    pub fn get_router(&self) -> &Router {
        &self.router
    }

    fn listen() -> SocketAddr {
        let listen = G_CONFIG.bin.get("api").unwrap().listen.as_str();
        let port = G_CONFIG.bin.get("api").unwrap().port;

        SocketAddr::from((IpAddr::from_str(listen).unwrap(), port))
    }

    fn router() -> Router {
        Router::new()
            .nest("/api/v1", routes::api_v1())
            .route("/health", get(|| async { "success" }))
            .fallback(|| async {
                Response::<String>::new(Some(404), Some("Not Found".to_string()), None)
            })
            .layer(
                ServiceBuilder::new()
                    .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
                    .layer(
                        TraceLayer::new_for_http()
                            .make_span_with(RequestIdMakeSpan)
                            .on_failure(OnFailureBehaviour),
                    )
                    .layer(axum::middleware::from_fn(middleware::log_response))
                    .layer(axum::middleware::from_fn(middleware::log_request))
                    .layer(PropagateRequestIdLayer::x_request_id())
                    .layer(CorsLayer::permissive()),
            )
    }
}

#[derive(Debug, Clone)]
struct RequestIdMakeSpan;

impl<B> MakeSpan<B> for RequestIdMakeSpan {
    fn make_span(&mut self, request: &Request<B>) -> Span {
        let request_id = request
            .extensions()
            .get::<RequestId>()
            .map(|request_id| request_id.header_value().to_str().unwrap())
            .unwrap_or_else(|| "unknown");

        let span = info_span!("root", request_id);

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

        span
    }
}

#[derive(Debug, Clone)]
struct OnFailureBehaviour;

impl<FailureClass> OnFailure<FailureClass> for OnFailureBehaviour
where
    FailureClass: Debug,
{
    fn on_failure(&mut self, failure_classification: FailureClass, latency: Duration, _: &Span) {
        error!(?failure_classification, ?latency, "<-- 请求处理失败",)
    }
}
