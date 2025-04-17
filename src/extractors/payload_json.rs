use crate::error::api::ApiError;
use crate::models::response::ApiResponse;
use axum::extract::rejection::JsonRejection;
use axum::extract::FromRequest;
use axum::response::{IntoResponse, Response};
#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(ApiError))]
pub struct PayloadJson<T>(pub T);


pub enum JsonError {
    // The request body contained invalid JSON
    JsonRejection(JsonRejection),
}

impl IntoResponse for JsonError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            JsonError::JsonRejection(rejection) => (rejection.status(), rejection.body_text())
        };

        (status,
            ApiResponse::<(), ()> {
                success: false,
                message: msg,
                data: None,
                error: None,
            }
        ).into_response()
    }
}