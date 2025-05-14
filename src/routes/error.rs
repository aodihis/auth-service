use axum::Json;
use axum::response::IntoResponse;
use http::StatusCode;
use serde_json::json;

pub async fn not_found_handler() -> impl IntoResponse {
    let body = json!({ "error": "Resource not found" });
    (StatusCode::NOT_FOUND, Json(body))
}
