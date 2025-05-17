#[allow(dead_code)]
#[allow(unused_variables)]
use thiserror::Error;
use crate::error::api::ApiError;

#[allow(dead_code)]
#[allow(unused_variables)]
#[derive(Error, Debug)]
pub enum AuthenticationError {
    #[error("Email or username is already registered")]
    AccountAlreadyExists,

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Internal server error")]
    InternalServerError,

    #[error("Invalid token")]
    InvalidToken,

    #[error("Invalid Credentials")]
    InvalidCredentials,
}