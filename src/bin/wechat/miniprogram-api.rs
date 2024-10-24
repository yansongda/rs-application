use tracing::info;
use application::config::Config;
use application::logger::Logger;
use application::wechat_miniprogram_api::App;

#[tokio::main]
async fn main() {
    Config::init();
    Logger::non_blocking("wechat-miniprogram-api");

    let app = App::init().await;

    let listener = tokio::net::TcpListener::bind(app.get_listen())
        .await
        .unwrap();

    info!("Listening on {}", app.get_listen());

    axum::serve(listener, app.get_router().clone())
        .await
        .unwrap();
}
