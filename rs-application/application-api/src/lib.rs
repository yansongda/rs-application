use application_kernel::config::G_CONFIG;
use salvo::catcher::Catcher;
use salvo::prelude::{Logger, RequestId};
use salvo::{Router, Service};
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

use crate::middleware::cors;

mod middleware;
mod request;
mod response;
mod routes;
mod service;
mod v1;

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
        let router = Router::new().push(routes::health()).push(routes::api_v1());

        Service::new(router)
            .hoop(Logger::new())
            .hoop(cors())
            .hoop(RequestId::new())
            .catcher(Catcher::default().hoop(routes::catcher))
    }
}
