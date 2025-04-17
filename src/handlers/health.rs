use axum::response::IntoResponse;
use crate::models::response::ApiResponse;

pub async fn health_handler() -> impl IntoResponse {
    ApiResponse::<(), ()> {
        success: true,
        message: "Service online".to_string(),
        data: None,
        error: None,
    }
}