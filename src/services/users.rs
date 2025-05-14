use std::sync::Arc;
use sqlx::{Error, PgPool};
use sqlx::postgres::PgPoolOptions;
use tracing::log::error;
use uuid::Uuid;
use crate::config::Config;
use crate::error::authentication::AuthenticationError;
use crate::error::user::UserError;
use crate::models::request::RegisterUser;
use crate::models::user::User;
use crate::services::traits::EmailServiceBase;
use crate::utils::security::hash_password;

pub struct Users {
    pool: PgPool,
    config: Arc<Config>
}

impl Users {
    pub fn new(pool: PgPool,  config: Arc<Config>) -> Self {
        Self {
            pool,
            config
        }
    }

    pub async fn create_user(&self, user_payload: RegisterUser) -> Result<User, UserError> {
        let user_id = Uuid::new_v4();
        let password_hash = match hash_password(&user_payload.password) {
            Ok(hash) => hash,
            Err(_) => return Err(UserError::InternalServerError),
        };
        let is_active = false;
        match sqlx::query(
            r#"
            INSERT INTO users (id, username, email, password_hash, is_active)
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
            .bind(user_id)
            .bind(user_payload.username)
            .bind(user_payload.email.clone())
            .bind(password_hash)
            .bind(is_active)
            .execute(&self.pool)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => {
                if let Error::Database(db_err) = &e {
                    if db_err.constraint() == Some("users_username_key") ||
                        db_err.constraint() == Some("users_email_key") {
                        return Err(UserError::AccountAlreadyExists)
                    }
                }
                error!("Failed to save users: {}", e.to_string());
                return Err(UserError::InternalServerError)
            }
        }?;
        Ok(User {
            id: user_id,
            email: user_payload.email,
            username: "".to_string(),
            is_active,
            created_at: Default::default(),
            updated_at: Default::default(),
        })
    }
}