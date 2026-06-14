use axum::{
    Json, Router,
    body::Body,
    http::{HeaderMap, Request},
    routing::{get, post},
};
use clap::Parser;
use http_body_util::BodyExt;
use serde::Deserialize;
use serde_json::{Value, json};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use std::{collections::HashMap, net::SocketAddr, path::Path, str::FromStr};
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
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let pool = connect("sqlite://webhook_replayer.db").await?;

    info!("Connected to SQLite database");

    create_tables(&pool).await?;

    info!("Database tables created or verified successfully");

    let app = Router::new()
        .route("/", get(hello_world))
        .route("/heathz", get(healthz))
        .fallback(webhook_handler);

    let listener = tokio::net::TcpListener::bind(args.listen).await.unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn hello_world() -> &'static str {
    "Hello, World!"
}

async fn healthz() -> &'static str {
    "OK"
}

async fn webhook_handler(req: Request<Body>) -> Result<Json<Value>, Json<Value>> {
    // Log the received webhook event
    let path = req.uri().path();

    let query = req.uri().query();

    let method = req.method().to_string();

    let headers = req.headers().clone();

    let byte_body = req
        .into_body()
        .collect()
        .await
        .map_err(|_| {
            error!("collect request body failed");
            Json(json!({"status": "error", "message": "Failed to read request body"}))
        })?
        .to_bytes();

    let payload: HashMap<String, Value> = serde_json::from_slice(&byte_body).unwrap_or_else(|_| {
        error!("Failed to parse request body as JSON");
        HashMap::new()
    });

    info!(
        "Received webhook event with headers: {:?} and payload: {:?}",
        headers, payload
    );

    // Here you would typically save the event to the database and forward it to the target URL

    Ok(Json(json!(payload)))
}

async fn connect(filename: &str) -> Result<SqlitePool, sqlx::Error> {
    let options = SqliteConnectOptions::from_str(filename)?.create_if_missing(true);

    SqlitePool::connect_with(options).await
}

async fn save_webhook_event(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Implement the logic to save the webhook event to the database
    Ok(())
}

async fn create_tables(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS webhook_replays (
            id INTEGER PRIMARY KEY AUTOINCREMENT,

            event_id INTEGER NOT NULL,
            target_url TEXT NOT NULL,

            request_headers_json TEXT NOT NULL,
            request_body BLOB NOT NULL,

            response_status INTEGER,
            response_headers_json TEXT,
            response_body BLOB,
            error TEXT,

            replayed_at TEXT NOT NULL,

            FOREIGN KEY(event_id) REFERENCES webhook_events(id)
        );

        CREATE TABLE IF NOT EXISTS webhook_events (
            id INTEGER PRIMARY KEY AUTOINCREMENT,

            method TEXT NOT NULL,
            path TEXT NOT NULL,
            query_string TEXT,

            headers_json TEXT NOT NULL,
            body BLOB NOT NULL,

            content_type TEXT,
            body_size INTEGER NOT NULL,

            received_at TEXT NOT NULL,

            forward_target TEXT,
            forward_status INTEGER,
            forward_response_headers_json TEXT,
            forward_response_body BLOB,
            forward_error TEXT,
            forwarded_at TEXT
        );
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}
