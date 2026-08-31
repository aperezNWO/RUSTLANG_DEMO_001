
use axum::{extract::Query, http::StatusCode, Json};
use tracing::{error, info};
use crate::{algorithms::run_random_dijkstra, engine::{generate_julia, generate_leaf, generate_mandelbrot}, models::{AccessLog, Bounds, FractalParams, FractalPoint, PersonaTable, fetch_access_logs, fetch_persons}};


pub async fn ping() -> StatusCode {
    info!("ping() called.");
    StatusCode::NO_CONTENT
}

pub async fn get_all_logs() -> Result<Json<Vec<AccessLog>>, (StatusCode, String)> {
    info!("get_all_logs() called.");
    fetch_access_logs().await.map(Json).map_err(|e| {
        error!("Database error: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })
}

pub async fn get_all_persons() -> Result<Json<Vec<PersonaTable>>, (StatusCode, String)> {
    info!("get_all_persons() called.");
    fetch_persons().await.map(Json).map_err(|e| {
        error!("Database error: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })
}

pub async fn get_fractal_handler(Query(params): Query<FractalParams>) -> Json<Vec<FractalPoint>> {

    let max_iterations = params.max_iterations.unwrap_or(500);

    let default_bounds = if params.kind == 1 {
        Bounds { x_min: -2.0, x_max: 1.0, y_min: -1.2, y_max: 1.2 }
    } else {
        Bounds { x_min: -1.5, x_max: 1.5, y_min: -1.5, y_max: 1.5 }
    };

    let bounds = match (params.x_min, params.x_max, params.y_min, params.y_max) {
        (Some(x_min), Some(x_max), Some(y_min), Some(y_max)) => Bounds { x_min, x_max, y_min, y_max },
        _ => default_bounds,
    };
    
    info!(
        "get_fractal_handler() called, kind: {:?}, bounds: {:?}, max_iterations: {}",
        params.kind, bounds, max_iterations
    );

    let points = match params.kind {
        1 => generate_mandelbrot(bounds, max_iterations),
        2 => generate_julia(bounds, max_iterations),
        3 => generate_leaf(),
        _ => vec![],
    };

    Json(points)
}

pub async fn generate_random_vertex() -> String {

    info!("generate_random_vertex() called.");
    
    run_random_dijkstra()
}