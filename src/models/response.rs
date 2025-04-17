use axum::Json;
use axum::response::{IntoResponse, Response};
use http::StatusCode;
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T, E>
where T: Serialize, E: Serialize
{
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
    pub error: Option<E>,
}

impl<T: Serialize, E: Serialize> IntoResponse for ApiResponse<T, E> {
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
pub struct ErrorFieldDetail {
    pub(crate) field: String,
    pub(crate) message: String,
}