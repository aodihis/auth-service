use std::sync::Arc;
use sqlx::{Error, PgPool};
use sqlx::postgres::{PgDatabaseError, PgQueryResult};
use tracing::{debug, error};
use crate::config::Config;
use crate::error::authorization::AuthorizationError;
use crate::error::user::UserError;
use crate::models::request::Permission;

pub struct Permissions {
    pool: PgPool,
    config: Arc<Config>,
}

impl Permissions {
    pub fn new(pool: PgPool, config: Arc<Config>) -> Self {
        Permissions { pool, config }
    }

    pub async fn add(&self, permission: Permission) -> Result<(), AuthorizationError> {

        let res = sqlx::query("INSERT INTO permissions (name, description, resource, action) VALUES($1, $2, $3, $4)")
            .bind(format!("{}:{}", permission.resource, permission.action))
            .bind(permission.description)
            .bind(permission.resource)
            .bind(permission.action)
            .execute(&self.pool).await;

        match res {
            Ok(_) => {
                Ok(())
            }
            Err(Error::Database(db_err)) => {
                if let pg_err = db_err.downcast_ref::<PgDatabaseError>() {
                    debug!("{}", pg_err.to_string());
                    if pg_err.code() == "23505" {
                        // Unique constraint violation
                        return Err(AuthorizationError::PermissionAlreadyExist);
                    }
                }
                error!("{}", db_err.to_string());
                Err(AuthorizationError::InternalServerError)
            },
            Err(e) => {
                error!("{}", e.to_string());
                Err(AuthorizationError::InternalServerError)
            },
        }
    }
}
