use axum::{Json, http::StatusCode};
use crate::models::SystemInfo;
use crate::system;
use tracing::info;

pub async fn get_system_info() -> Result<(StatusCode, Json<SystemInfo>), (StatusCode, Json<serde_json::Value>)> {
    info!("fetching sysinfo");

    let system_info = system::get_system_info();

    Ok((StatusCode::OK, Json(system_info)))
} 