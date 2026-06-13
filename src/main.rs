use axum::{Router, routing::get};
use clap::Parser;
use std::net::SocketAddr;
use tracing::{Level, error, info};

#[derive(Parser, Debug)]
#[command(version, about = "A webhook-replayer for webhook replay purposes")]
struct Args {
    #[arg(short, long)]
    listen: SocketAddr,

    #[arg(short, long)]
    forward: SocketAddr,
}

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

    let args = match Args::try_parse() {
        Ok(parsed_args) => parsed_args,
        Err(error) => {
            error!("Error parsing arguments: {}", error);
            std::process::exit(1);
        }
    };

    info!(
        "Starting webhook-replayer with listen: {} and forward: {}",
        args.listen, args.forward
    );

    let app = Router::new()
        .route("/", get(hello_world))
        .route("/heathz", get(healthz));

    let listener = tokio::net::TcpListener::bind(args.listen).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> &'static str {
    "Hello, World!"
}

async fn healthz() -> &'static str {
    "OK"
}
