use axum::Json;
use axum::response::{IntoResponse, Response};
use http::StatusCode;
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RegisterError {
    #[error("Email is already registered")]
    EmailAlreadyExists,

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Internal server error")]
    InternalServerError,
}

impl IntoResponse for RegisterError {
    fn into_response(self) -> Response {
        let status = match self {
            RegisterError::EmailAlreadyExists => StatusCode::CONFLICT,
            RegisterError::InvalidInput(_) => StatusCode::BAD_REQUEST,
            RegisterError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = Json(json!({
            "error": self.to_string(),
        }));

        (status, body).into_response()
    }
}