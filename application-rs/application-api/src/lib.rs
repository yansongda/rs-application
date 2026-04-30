use crate::middleware::request_logger;
use application_kernel::config::G_CONFIG;
use salvo::catcher::Catcher;
use salvo::cors::{AllowOrigin, Cors};
use salvo::http::Method;
use salvo::prelude::RequestId;
use salvo::{Router, Service};
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

mod middleware;
mod request;
mod response;
mod routes;
mod service;
mod v1;

pub struct App;

impl App {
    pub fn listen() -> SocketAddr {
        let api_config = G_CONFIG
            .bin
            .get("api")
            .expect("配置中缺少 'api' 配置项");

        let listen = api_config.listen.as_str();
        let port = api_config.port;

        SocketAddr::from((
            IpAddr::from_str(listen).expect("API 监听地址格式无效"),
            port,
        ))
    }

    pub fn router() -> Service {
        let router = Router::new().push(routes::health()).push(routes::api_v1());

        Service::new(router)
            .hoop(RequestId::new())
            .hoop(request_logger)
            .hoop(
                Cors::new()
                    .allow_origin(AllowOrigin::any())
                    .allow_methods(vec![Method::GET, Method::POST, Method::DELETE])
                    .allow_headers("authorization")
                    .into_handler(),
            )
            .catcher(Catcher::default().hoop(routes::catcher))
    }
}
