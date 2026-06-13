use axum::{Router, routing::get};
use tracing::{Level, info};

#[tokio::main]
async fn main() {
    // Initialize the subscriber with JSON formatting
    tracing_subscriber::fmt()
        .json()
        .with_max_level(Level::INFO)
        .with_target(true) // Include the module path
        .with_current_span(true) // Include the current span
        .with_span_list(true) // Include the full span hierarchy
        .init();

    let app = Router::new().route("/", get(hello_world));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3210")
        .await
        .unwrap();

    info!("Listening on: http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> &'static str {
    "Hello, World!"
}
