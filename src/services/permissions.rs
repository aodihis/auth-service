use crate::config::Config;
use crate::error::authorization::AuthorizationError;
use crate::models::authorization::Permission as PermissionModel;
use crate::models::request::Permission;
use sqlx::postgres::PgDatabaseError;
use sqlx::{Error, PgPool};
use std::sync::Arc;
use tracing::{debug, error, info};

pub struct Permissions {
    pool: PgPool,
    config: Arc<Config>,
}

impl Permissions {
    pub fn new(pool: PgPool, config: Arc<Config>) -> Self {
        Permissions { pool, config }
    }

    pub async fn add(&self, permission: Permission) -> Result<PermissionModel, AuthorizationError> {
        let res = sqlx::query_as(
            "INSERT INTO permissions (name, description, resource, action) VALUES($1, $2, $3, $4)\
                RETURNING *
            ",
        )
        .bind(format!("{}:{}", permission.resource, permission.action))
        .bind(permission.description)
        .bind(permission.resource)
        .bind(permission.action)
        .fetch_one(&self.pool)
        .await;

        match res {
            Ok(permission) => Ok(permission),
            Err(Error::Database(db_err)) => {
                let pg_err = db_err.downcast_ref::<PgDatabaseError>();

                if pg_err.code() == "23505" {
                    debug!("Duplicate permission: {}", pg_err.to_string());
                    return Err(AuthorizationError::PermissionAlreadyExist);
                }

                error!("{}", db_err.to_string());
                Err(AuthorizationError::InternalServerError)
            }
            Err(e) => {
                error!("{}", e.to_string());
                Err(AuthorizationError::InternalServerError)
            }
        }
    }

    pub async fn delete(&self, id: i32) -> Result<(), AuthorizationError> {
        let res = sqlx::query("DELETE FROM permissions WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await;

        match res {
            Ok(res) => {
                if res.rows_affected() == 0 {
                    debug!("Delete failed: Permission with id {} not found", id);
                    Err(AuthorizationError::NotFound)
                } else {
                    Ok(())
                }
            }
            Err(e) => {
                error!(
                    "Database error while deleting permission id {}: {:?}",
                    id, e
                );
                Err(AuthorizationError::InternalServerError)
            }
        }
    }

    pub async fn list(&self) -> Result<Vec<PermissionModel>, AuthorizationError> {
        let res: Result<Vec<PermissionModel>, Error> = sqlx::query_as("SELECT * FROM permissions")
            .fetch_all(&self.pool)
            .await;

        match res {
            Ok(rows) => Ok(rows),
            Err(err) => {
                error!("{}", err.to_string());
                Err(AuthorizationError::InternalServerError)
            }
        }
    }

    pub async fn update(
        &self,
        id: i32,
        permission: Permission,
    ) -> Result<PermissionModel, AuthorizationError> {
        let res = sqlx::query_as(
            "UPDATE permissions SET name = $1, description = $2,
                resource = $3, action = $4
                WHERE id = $5
                RETURNING *
            ",
        )
        .bind(format!("{}:{}", permission.resource, permission.action))
        .bind(permission.description)
        .bind(permission.resource)
        .bind(permission.action)
        .bind(id)
        .fetch_optional(&self.pool)
        .await;

        match res {
            Ok(Some(permission)) => Ok(permission),
            Ok(None) => {
                info!("Update failed: role not found: {}", id);
                Err(AuthorizationError::NotFound)
            }
            Err(Error::Database(db_err)) => {
                let pg_err = db_err.downcast_ref::<PgDatabaseError>();
                if pg_err.code() == "23505" {
                    debug!("Failed to update permission id {}: duplicate", id);
                    return Err(AuthorizationError::PermissionAlreadyExist);
                }

                error!(
                    "Failed to update permission id {}: {}",
                    id,
                    db_err.to_string()
                );
                Err(AuthorizationError::InternalServerError)
            }
            Err(e) => {
                error!("Failed to update permission id {}: {}", id, e.to_string());
                Err(AuthorizationError::InternalServerError)
            }
        }
    }
}
