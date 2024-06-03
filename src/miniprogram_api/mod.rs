use std::fmt::Debug;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::time::Duration;

use axum::http::Request;
use axum::routing::get;
use axum::{http, Router};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::request_id::{
    MakeRequestUuid, PropagateRequestIdLayer, RequestId, SetRequestIdLayer,
};
use tower_http::trace::{MakeSpan, OnFailure, OnRequest, OnResponse, TraceLayer};
use tracing::metadata::LevelFilter;
use tracing::{error, info, info_span, Event, Span, Subscriber};
use tracing_appender::non_blocking::{NonBlockingBuilder, WorkerGuard};
use tracing_subscriber::filter;
use tracing_subscriber::fmt::{format, FmtContext, FormatEvent, FormatFields, FormattedFields};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry::{LookupSpan, Scope};
use tracing_subscriber::util::SubscriberInitExt;

use crate::config::Config;
use crate::model::result::Response;
use crate::repository::Pool;

mod extract;
mod middleware;
mod response;
mod routes;
mod v1;

pub struct App {
    listen: SocketAddr,
    router: Router,
}

impl App {
    pub async fn init() -> Self {
        Pool::init().await;

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
        let listen = Config::get_bin("miniprogram_api").listen.as_str();
        let port = Config::get_bin("miniprogram_api").port;

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
                            .on_request(OnRequestBehaviour)
                            .on_response(OnResponseBehaviour)
                            .on_failure(OnFailureBehaviour),
                    )
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

        info_span!("root", request_id)
    }
}

#[derive(Debug, Clone)]
struct OnRequestBehaviour;

impl<B: Debug> OnRequest<B> for OnRequestBehaviour {
    fn on_request(&mut self, request: &Request<B>, _: &Span) {
        info!(
            method = %request.method(),
            uri = %request.uri(),
            headers = ?request.headers(),
            inputs = ?request.body(),
            "--> 接收到请求",
        );
    }
}

#[derive(Debug, Clone)]
struct OnResponseBehaviour;

impl<B: Debug> OnResponse<B> for OnResponseBehaviour {
    fn on_response(self, _: &http::Response<B>, latency: Duration, _: &Span) {
        let elapsed = latency.as_secs_f32();

        info!(elapsed, "<-- 请求处理完成");
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
