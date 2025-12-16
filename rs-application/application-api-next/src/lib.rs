use application_kernel::config::G_CONFIG;
use salvo::{Router, Service};
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use salvo::catcher::Catcher;
use salvo::cors::Cors;
use salvo::http::Method;
use salvo::prelude::RequestId;

mod routes;
mod response;

pub struct App {
    listen: SocketAddr,
    router: Service,
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

    pub fn get_router(&self) -> &Service {
        &self.router
    }

    fn listen() -> SocketAddr {
        let listen = G_CONFIG.bin.get("api").unwrap().listen.as_str();
        let port = G_CONFIG.bin.get("api").unwrap().port;

        SocketAddr::from((IpAddr::from_str(listen).unwrap(), port))
    }

    fn router() -> Service {
        let router = Router::new()
            .push(routes::api_v1())
            .push(routes::health());

        let cors = Cors::new()
            .allow_origin(["*"])
            .allow_methods(vec![Method::GET, Method::POST, Method::DELETE])
            .allow_headers("authorization")
            .into_handler();

        Service::new(router).hoop(cors).hoop(RequestId::new()).catcher(Catcher::default().hoop())

        // Router::new()
        //     .nest("/api/v1", routes::api_v1())
        //     .route("/health", get(|| async { "success" }))
        //     .fallback(|| async {
        //         Response::<String>::new(Some(404), Some("Not Found".to_string()), None)
        //     })
        //     .layer(
        //         ServiceBuilder::new()
        //             .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        //             .layer(
        //                 TraceLayer::new_for_http()
        //                     .make_span_with(RequestIdMakeSpan)
        //                     .on_failure(OnFailureBehaviour),
        //             )
        //             .layer(axum::middleware::from_fn(middleware::log_response))
        //             .layer(axum::middleware::from_fn(middleware::log_request))
        //             .layer(PropagateRequestIdLayer::x_request_id())
        //             .layer(CorsLayer::permissive()),
        //     )
    }
}
