use axum::Json;
use axum::response::{IntoResponse, Response};
use http::StatusCode;
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RegisterError {
    #[error("Email or username is already registered")]
    AccountAlreadyExists,

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Internal server error")]
    InternalServerError,
}

impl IntoResponse for RegisterError {
    fn into_response(self) -> Response {
        let status = match self {
            RegisterError::AccountAlreadyExists => StatusCode::CONFLICT,
            RegisterError::InvalidInput(_) => StatusCode::BAD_REQUEST,
            RegisterError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        };


        let body = match self {

            RegisterError::InvalidInput(message) => {
                Json(json!({ "error": "Invalid input", "details": message }))
            }
            _ => {
                Json(json!({
                    "error": self.to_string(),
                }))
            }
        };
        (status, body).into_response()
    }
}