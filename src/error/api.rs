use std::fmt;
use crate::models::response::{ApiResponse, ErrorFieldDetail};
use axum::response::{IntoResponse, Response};
use http::StatusCode;

pub enum ApiError {
    Conflict(String),
    Unauthorized(String),
    BadRequest(String),
    InternalServerError(String),
    ValidationError {
        message: String,
        field_errors: Vec<(String, String)>,
    },
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Conflict(msg) => write!(f, "Conflict: {}", msg),
            Self::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            Self::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            Self::InternalServerError(msg) => write!(f, "Internal server error: {}", msg),
            Self::ValidationError { message, .. } => write!(f, "Validation error: {}", message),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let details = self.to_string();
        let error_response: ApiResponse<(), String> = ApiResponse {
            success: false,
            message: self.to_string(),
            data: None,
            error: Some(details),
        };
        (status, error_response).into_response()
    }
}

impl ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::Conflict(_) => {StatusCode::CONFLICT}
            ApiError::Unauthorized(_) => {StatusCode::UNAUTHORIZED}
            ApiError::BadRequest(_) => {StatusCode::BAD_REQUEST}
            ApiError::InternalServerError(_) => {StatusCode::INTERNAL_SERVER_ERROR}
            ApiError::ValidationError { .. } => {StatusCode::UNPROCESSABLE_ENTITY}
        }
    }

    fn details(&self) -> Vec<ErrorFieldDetail> {
        match self {
            Self::ValidationError { field_errors, .. } => {
                field_errors
                    .iter()
                    .map(|(field, message)| ErrorFieldDetail {
                        field: field.clone(),
                        message: message.clone(),
                    })
                    .collect()
            }
            _ => Vec::new(),
        }
    }
}