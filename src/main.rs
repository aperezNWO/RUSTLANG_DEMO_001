mod algorithms;
mod db;
mod engine;
mod handlers;
mod models;

use axum::{
    http::Method,
    routing::get,
    Router,
};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers(Any);

    let app = Router::new()
        .route("/ping", get(handlers::ping))
        .route("/api/fractals/generate", get(handlers::get_fractal_handler))
        .route("/GenerateRandomVertex_SpringBoot", get(handlers::generate_random_vertex))
        .route("/api/data/getAllLogs", get(handlers::get_all_logs))
        .route("/api/data/getAllPersons", get(handlers::get_all_persons))
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    info!("Rust API Server initializing on port 8080...");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}