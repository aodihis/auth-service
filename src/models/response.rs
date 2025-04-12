use axum::Json;
use axum::response::{IntoResponse, Response};
use http::StatusCode;
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
    pub error: Option<ApiError>,
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        let status = if self.success {
            StatusCode::OK
        } else {
            StatusCode::BAD_REQUEST // or customize it based on `self.error.code`
        };

        (status, Json(self)).into_response()
    }
}


#[derive(Serialize)]
pub struct ApiError {
    pub code: String,
    pub details: Option<String>,
}
