use crate::models::response::{ApiResponse, ErrorFieldDetail};
use axum::extract::rejection::JsonRejection;
use axum::response::{IntoResponse, Response};
use http::StatusCode;
#[allow(dead_code)]
#[allow(unused_variables)]
use std::fmt;

#[allow(dead_code)]
#[allow(unused_variables)]
pub enum ApiError {
    Conflict(String),
    Unauthorized(String),
    BadRequest(String),
    InternalServerError(String),
    ValidationError {
        message: String,
        field_errors: Vec<(String, String)>,
    },
    JsonRejection(JsonRejection),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Conflict(msg) => write!(f, "Conflict: {}", msg),
            Self::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            Self::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            Self::InternalServerError(msg) => write!(f, "Internal server error: {}", msg),
            Self::ValidationError { message, .. } => write!(f, "Validation error: {}", message),
            Self::JsonRejection(rejection) => write!(f, "{}", rejection.body_text()),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let details = self.details();
        let error_response: ApiResponse<(), Vec<ErrorFieldDetail>> = ApiResponse {
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
            ApiError::JsonRejection { .. } => {StatusCode::BAD_REQUEST}
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

impl From<JsonRejection> for ApiError {
    fn from(error: JsonRejection) -> Self {
        Self::JsonRejection(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;
    use axum::response::IntoResponse;
    use http::StatusCode;
    use serde_json::{json, Value};


    // Dummy implementation of ErrorFieldDetail if needed
    #[derive(Debug, serde::Serialize, PartialEq)]
    struct ErrorFieldDetail {
        field: String,
        message: String,
    }

    // Dummy implementation of ApiResponse if needed
    #[derive(Debug, serde::Serialize)]
    struct ApiResponse<T, E> {
        success: bool,
        message: String,
        data: Option<T>,
        error: Option<E>,
    }

    #[tokio::test]
    async fn test_conflict_error_response() {
        let err = ApiError::Conflict("Item already exists".into());
        let response = err.into_response();

        assert_eq!(response.status(), StatusCode::CONFLICT);
        let body = to_bytes(response.into_body(), 100).await.unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["success"], false);
        assert_eq!(json["message"], "Conflict: Item already exists");
        assert_eq!(json["data"], Value::Null);
        assert_eq!(json["error"], json!([]));
    }

    #[tokio::test]
    async fn test_validation_error_response() {
        let err = ApiError::ValidationError {
            message: "Invalid input".into(),
            field_errors: vec![
                ("username".into(), "Username is required".into()),
                ("email".into(), "Email is invalid".into()),
            ],
        };
        let response = err.into_response();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
        let body = to_bytes(response.into_body(), 300).await.unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["success"], false);
        assert_eq!(json["message"], "Validation error: Invalid input");
        assert_eq!(json["data"], Value::Null);

        let expected_errors = json!([
            { "field": "username", "message": "Username is required" },
            { "field": "email", "message": "Email is invalid" }
        ]);
        assert_eq!(json["error"], expected_errors);
    }

    #[tokio::test]
    async fn test_internal_server_error_response() {
        let err = ApiError::InternalServerError("Something went wrong".into());
        let response = err.into_response();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let body = to_bytes(response.into_body(), 100).await.unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["success"], false);
        assert_eq!(json["message"], "Internal server error: Something went wrong");
    }

    #[tokio::test]
    async fn test_bad_request_error_response() {
        let err = ApiError::BadRequest("Invalid query".into());
        let response = err.into_response();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body = to_bytes(response.into_body(), 100).await.unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["success"], false);
        assert_eq!(json["message"], "Bad request: Invalid query");
    }

    #[tokio::test]
    async fn test_unauthorized_error_response() {
        let err = ApiError::Unauthorized("Login required".into());
        let response = err.into_response();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        let body = to_bytes(response.into_body(), 100).await.unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["success"], false);
        assert_eq!(json["message"], "Unauthorized: Login required");
    }

}
