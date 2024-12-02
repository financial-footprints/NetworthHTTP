mod config;
mod controllers;

#[tokio::main]
async fn main() {
    let config = config::get_config().await;
    let routes = controllers::routes(config);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8001")
        .await
        .unwrap();
    axum::serve(listener, routes).await.unwrap();
}
