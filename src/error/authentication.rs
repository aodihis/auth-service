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

