mod algorithms;
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
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() {
    // Enable INFO logging for both your binary crate and tower_http middleware
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "rust_demo_001=info,tower_http=info,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

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