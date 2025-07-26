use crate::{app::AppState, errors::ApiResult};
use axum::{extract::State, response::Json};
use serde_json::{json, Value};

pub async fn health_check_handler(
    State(_state): State<AppState>,
) -> ApiResult<Json<Value>> {
    Ok(Json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "service": "axum-api-template"
    })))
}

pub async fn readiness_check_handler(
    State(state): State<AppState>,
) -> ApiResult<Json<Value>> {
    match state.pool.get() {
        Ok(_) => Ok(Json(json!({
            "status": "ready",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "database": "connected"
        }))),
        Err(_) => Ok(Json(json!({
            "status": "not ready",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "database": "disconnected"
        })))
    }
}
