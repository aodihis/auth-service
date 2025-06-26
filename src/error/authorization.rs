use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthorizationError {
    #[error("Permission exist")]
    PermissionAlreadyExist,

    #[error("Permission not found")]
    PermissionNotFound,

    #[error("Role exist")]
    RoleAlreadyExist,

    #[error("Role not found")]
    RoleNotFound,

    #[error("Internal Server Error")]
    InternalServerError,

    #[error("Not found")]
    NotFound,

    #[error("ForeignKeyViolation")]
    ForeignKeyViolation,
}
