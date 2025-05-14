use crate::models::response::ApiResponse;
use axum::response::IntoResponse;

pub async fn health_handler() -> impl IntoResponse {
    ApiResponse::<(), ()> {
        success: true,
        message: "Service online".to_string(),
        data: None,
        error: None,
    }
}
