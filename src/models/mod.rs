use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessLog {
    pub id_column: i64,
    pub page_name: Option<String>,
    pub access_date: Option<String>,
    pub ip_value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonaTable {
    pub id_column: i64,
    pub ciudad: Option<String>,
    pub nombre_completo: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FractalPoint {
    pub x: f64,
    pub y: f64,
    pub intensity: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct Bounds {
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
}

#[derive(Debug, Deserialize)]
pub struct FractalParams {
    pub kind: i32,
    pub x_min: Option<f64>,
    pub x_max: Option<f64>,
    pub y_min: Option<f64>,
    pub y_max: Option<f64>,
    pub max_iterations: Option<i32>,
}