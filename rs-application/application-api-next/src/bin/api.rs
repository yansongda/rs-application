use application_api_next::App;
use application_kernel::logger::Logger;
use tracing::info;

#[tokio::main]
async fn main() {
    let _logger = Logger::non_blocking("api");

    let app = App::init();

    let listener = tokio::net::TcpListener::bind(app.get_listen())
        .await
        .unwrap();

    info!("Listening on {}", app.get_listen());

    Server::new(listener).serve(app.get_router().clone()).await;
}
