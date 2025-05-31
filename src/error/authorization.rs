use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthorizationError {

    #[error("Permission exist")]
    PermissionAlreadyExist,

    #[error("Internal Server Error")]
    InternalServerError,

    #[error("Not found")]
    NotFound
}