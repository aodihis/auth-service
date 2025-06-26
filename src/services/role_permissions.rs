use crate::config::Config;
use crate::error::authorization::AuthorizationError;
use crate::models::authorization::Permission;
use sqlx::postgres::PgDatabaseError;
use sqlx::{Error, PgPool};
use std::sync::Arc;
use tracing::{error, info};

pub struct RolePermissions {
    pool: PgPool,
    config: Arc<Config>,
}

impl RolePermissions {
    pub fn new(pool: PgPool, config: Arc<Config>) -> Self {
        Self { pool, config }
    }

    pub async fn get_role_permissions(
        &self,
        role_id: i32,
    ) -> sqlx::Result<Vec<Permission>, AuthorizationError> {
        let res = sqlx::query_as::<_, Permission>(
            "
                SELECT p.*
                FROM permissions p
                INNER JOIN role_permissions rp ON rp.permission_id = p.id
                WHERE rp.role_id = $1
                ",
        )
        .bind(role_id)
        .fetch_all(&self.pool)
        .await;

        match res {
            Ok(permissions) => Ok(permissions),
            Err(error) => {
                error!("Failed to query role permissions: {}", error.to_string());
                Err(AuthorizationError::InternalServerError)
            }
        }
    }

    pub async fn add_permissions_for_role(
        &self,
        role_id: i32,
        permission_ids: Vec<i32>,
    ) -> sqlx::Result<(), AuthorizationError> {
        // Convert the permission_ids to a Postgres array
        let result = sqlx::query(
            "
                INSERT INTO role_permissions (role_id, permission_id)
                SELECT $1, UNNEST($2::int[])
                ON CONFLICT DO NOTHING
                ",
        )
        .bind(role_id)
        .bind(&permission_ids)
        .execute(&self.pool)
        .await;

        match result {
            Ok(_) => Ok(()),

            Err(Error::Database(db_err)) => {
                info!("Failed to add role permission: {}", db_err.to_string());
                let pg_err = db_err.downcast_ref::<PgDatabaseError>();
                if pg_err.code() == "23503" {
                    return match pg_err.constraint() {
                        Some("role_permissions_role_id_fkey") => {
                            Err(AuthorizationError::RoleNotFound)
                        }
                        Some("role_permissions_permission_id_fkey") => {
                            Err(AuthorizationError::PermissionNotFound)
                        }
                        _ => Err(AuthorizationError::ForeignKeyViolation),
                    };
                }
                Err(AuthorizationError::InternalServerError)
            }
            Err(e) => {
                error!("Failed to add role permission: {}", e.to_string());
                Err(AuthorizationError::InternalServerError)
            }
        }
    }

    pub async fn delete_permissions_for_role(
        &self,
        role_id: i32,
    ) -> sqlx::Result<(), AuthorizationError> {
        let result = sqlx::query("DELETE FROM role_permissions WHERE role_id = $1")
            .bind(role_id)
            .execute(&self.pool)
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => {
                error!("Failed to delete role permissions: {}", err.to_string());
                Err(AuthorizationError::InternalServerError)
            }
        }
    }
}
