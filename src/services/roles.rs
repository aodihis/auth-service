use std::sync::Arc;
use sqlx::PgPool;
use tracing::{debug, error, info};
use crate::config::Config;
use crate::error::authorization::AuthorizationError;
use crate::models::request::Role;
use crate::models::authorization::Role as RoleModel;

pub struct Roles {
    pool: PgPool,
    config: Arc<Config>,
}

impl Roles {

    pub fn new(pool: PgPool, config: Arc<Config>) -> Self {
        Self { pool, config }
    }

    pub async fn add(&self, role: Role) -> Result<RoleModel, AuthorizationError> {
        let res = sqlx::query_as::<_, RoleModel>(
            "INSERT INTO roles (name, description) VALUES ($1, $2)\
            RETURNING id, name, description"
        ).bind(role.name.clone()).bind(role.description).fetch_one(&self.pool).await;

        match res {
            Ok(role) => Ok(role),
            Err(err) => {
                if let sqlx::Error::Database(db_err) = &err {
                    info!("Insert role failed - Duplicated: Role with name {}", role.name);
                    if db_err.constraint() == Some("roles_name_key") {
                        return Err(AuthorizationError::RoleAlreadyExist);
                    }
                }
                error!("Insert role failed: {}", err.to_string());
                Err(AuthorizationError::InternalServerError)
            }
        }
    }

    pub async fn delete(&self, id: i32) -> Result<(), AuthorizationError> {
        let res = sqlx::query("DELETE FROM roles WHERE id = $1")
            .bind(id)
            .execute(&self.pool).await;

        match res {
            Ok(res) => {
                if res.rows_affected() == 0 {
                    info!("Delete failed: Role with id {} not found", id);
                    Err(AuthorizationError::NotFound)
                } else {
                    Ok(())
                }
            }
            Err(e) => {
                error!("Database error while deleting role id {}: {:?}", id, e);
                Err(AuthorizationError::InternalServerError)
            }
        }
    }
}