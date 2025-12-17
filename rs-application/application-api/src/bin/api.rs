use application_api::App;
use application_kernel::logger::Logger;
use salvo::prelude::TcpListener;
use salvo::{Listener, Server};

#[tokio::main]
async fn main() {
    let _logger = Logger::non_blocking("api");

    let listen = App::listen();
    let service = App::router();

    let listener = TcpListener::new(listen).bind().await;

    Server::new(listener).serve(service).await;
}
