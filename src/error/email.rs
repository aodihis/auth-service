use thiserror::Error;

#[allow(dead_code)]
#[allow(unused_variables)]
#[derive(Error, Debug)]
pub enum EmailError {
    #[error("Failed to connect to the email server")]
    ConnectionError,

    #[error("Invalid email address: {0}")]
    InvalidRecipient(String),

    #[error("SMTP protocol error: {0}")]
    SmtpError(String),

    #[error("Unexpected error: {0}")]
    Other(#[from] anyhow::Error),

    #[error("Internal server error")]
    InternalServerError
}