use thiserror::Error;
use crate::error::api::ApiError;

#[allow(dead_code)]
#[allow(unused_variables)]
#[derive(Error, Debug)]
pub enum UserError {
    #[error("Email or username is already registered")]
    AccountAlreadyExists,

    #[error("Internal server error")]
    InternalServerError,

    #[error("User not found: {0}")]
    UserNotFound(String),
}

impl Into<ApiError> for UserError {
    fn into(self) -> ApiError {
        match self {
            UserError::AccountAlreadyExists => ApiError::Conflict(String::from("AccountAlreadyExists")),
            UserError::InternalServerError => ApiError::InternalServerError("Internal server error".to_string()),
            UserError::UserNotFound(error) => ApiError::BadRequest(error),
        }
    }
}