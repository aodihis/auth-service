use axum::response::IntoResponse;
use axum::{Json, Router};
use axum::routing::get;
use serde_json::json;

pub fn router() -> Router {
    Router::new().route("/check", get(health_handler))
}

async fn health_handler() -> impl IntoResponse {
    Json(json!({"status": "ok"}))
}