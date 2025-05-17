use crate::error::api::ApiError;
use thiserror::Error;

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