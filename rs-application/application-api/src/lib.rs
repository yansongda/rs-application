use application_kernel::config::G_CONFIG;
use salvo::catcher::Catcher;
use salvo::cors::{AllowOrigin, Cors};
use salvo::http::Method;
use salvo::prelude::{Logger, RequestId};
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
        let listen = G_CONFIG.bin.get("api").unwrap().listen.as_str();
        let port = G_CONFIG.bin.get("api").unwrap().port;

        SocketAddr::from((IpAddr::from_str(listen).unwrap(), port))
    }

    pub fn router() -> Service {
        let router = Router::new().push(routes::health()).push(routes::api_v1());

        Service::new(router)
            .hoop(RequestId::new())
            .hoop(Logger::new())
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
