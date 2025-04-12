use crate::models::response::ApiResponse;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;

pub fn router() -> Router {
    Router::new().route("/check", get(health_handler))
}

async fn health_handler() -> impl IntoResponse {
    ApiResponse::<()> {
        success: true,
        message: "Service online".to_string(),
        data: None,
        error: None,
    }
}