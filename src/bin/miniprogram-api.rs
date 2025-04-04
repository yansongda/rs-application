use application::logger::Logger;
use application::miniprogram_api::App;
use tracing::info;

#[tokio::main]
async fn main() {
    let _logger = Logger::non_blocking("miniprogram-api");

    let app = App::init();

    let listener = tokio::net::TcpListener::bind(app.get_listen())
        .await
        .unwrap();

    info!("Listening on {}", app.get_listen());

    axum::serve(listener, app.get_router().clone())
        .await
        .unwrap();
}
