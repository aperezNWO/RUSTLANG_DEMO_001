mod algorithms;
mod engine;
mod handlers;
mod models;

use axum::{http::Method, response::IntoResponse, routing::get, Json, Router};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::LazyLock;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

// Estructura que replica EndpointInfo de C++
#[allow(dead_code)]
struct EndpointInfo {
    endpoint_name: &'static str,
    endpoint_description: &'static str,
}

// Diccionario global de endpoints equivalente al unordered_map de C++
static ENDPOINT_DICTIONARY: LazyLock<HashMap<&'static str, EndpointInfo>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert(
        "HEALTH_ENDPOINT",
        EndpointInfo {
            endpoint_name: "/health",
            endpoint_description: "Print all endpoints",
        },
    );
    m.insert(
        "PING_ENDPOINT",
        EndpointInfo {
            endpoint_name: "/ping",
            endpoint_description: "Render workaround",
        },
    );
    m.insert(
        "APP_VERSION_ENDPOINT",
        EndpointInfo {
            endpoint_name: "/api/version/getAppVersion",
            endpoint_description: "Get Application Version",
        },
    );
    m.insert(
        "STD_VERSION_ENDPOINT",
        EndpointInfo {
            endpoint_name: "/api/version/rust",
            endpoint_description: "Get Rust Language Version",
        },
    );
    m.insert(
        "SERVER_VERSION_ENDPOINT",
        EndpointInfo {
            endpoint_name: "/api/version/getServerVersion",
            endpoint_description: "Get Http Server Version",
        },
    );
    m.insert(
        "FRACTAL_ENDPOINT",
        EndpointInfo {
            endpoint_name: "/api/fractals/generate",
            endpoint_description: "Fractal Generation Endpoint",
        },
    );
    m.insert(
        "DIJKSTRA_ENDPOINT",
        EndpointInfo {
            endpoint_name: "/api/Algorithm/GenerateRandomVertex_Rust",
            endpoint_description: "Generate Random Vertex via Dijkstra",
        },
    );
    m.insert(
        "LOGS_ENDPOINT",
        EndpointInfo {
            endpoint_name: "/api/data/getAllLogs",
            endpoint_description: "Get All Logs",
        },
    );
    m.insert(
        "PERSONS_ENDPOINT",
        EndpointInfo {
            endpoint_name: "/api/data/getAllPersons",
            endpoint_description: "Get All Persons",
        },
    );
    m
});

// Estructuras serializables para garantizar el orden en el JSON ("server" primero, "endpoints" después)
#[derive(Serialize)]
struct EndpointEntry {
    key: &'static str,
    path: &'static str,
    description: &'static str,
}

#[derive(Serialize)]
struct HealthResponse {
    server: String,
    endpoints: Vec<EndpointEntry>,
}

// Manejador /health dinámico que lista el servidor y todos los endpoints del diccionario con orden garantizado
async fn health_handler() -> impl IntoResponse {
    let server_version = env!("CARGO_PKG_VERSION");
    let server_desc = format!("Rust Http Server 'axum' - v[{}] Working!", server_version);

    let endpoints_list: Vec<EndpointEntry> = ENDPOINT_DICTIONARY
        .iter()
        .map(|(key, info)| EndpointEntry {
            key,
            path: info.endpoint_name,
            description: info.endpoint_description,
        })
        .collect();

    let response_body = HealthResponse {
        server: server_desc,
        endpoints: endpoints_list,
    };

    Json(response_body)
}

#[tokio::main]
async fn main() {
    // Habilitar logging estructurado
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
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(Any);

    // Construcción del Router de Axum utilizando las rutas mapeadas desde el diccionario y handlers
    let app = Router::new()
        .route(
            ENDPOINT_DICTIONARY["PING_ENDPOINT"].endpoint_name,
            get(handlers::zero_handler),
        )
        .route(
            ENDPOINT_DICTIONARY["HEALTH_ENDPOINT"].endpoint_name,
            get(health_handler),
        )
        .route(
            ENDPOINT_DICTIONARY["FRACTAL_ENDPOINT"].endpoint_name,
            get(handlers::get_fractal_handler),
        )
        .route(
            ENDPOINT_DICTIONARY["DIJKSTRA_ENDPOINT"].endpoint_name,
            get(handlers::generate_random_vertex),
        )
        .route(
            ENDPOINT_DICTIONARY["LOGS_ENDPOINT"].endpoint_name,
            get(handlers::get_all_logs),
        )
        .route(
            ENDPOINT_DICTIONARY["PERSONS_ENDPOINT"].endpoint_name,
            get(handlers::get_all_persons),
        )
        .route(
            ENDPOINT_DICTIONARY["APP_VERSION_ENDPOINT"].endpoint_name,
            get(handlers::get_app_version),
        )
        .route(
            ENDPOINT_DICTIONARY["STD_VERSION_ENDPOINT"].endpoint_name,
            get(handlers::get_rust_version),
        )
        .route(
            ENDPOINT_DICTIONARY["SERVER_VERSION_ENDPOINT"].endpoint_name,
            get(handlers::get_server_version),
        )
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);

    info!("Rust API Server initializing on port {}...", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
