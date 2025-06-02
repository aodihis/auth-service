use crate::config::Config;
use crate::error::user::UserError;
use crate::models::request::RegisterUser;
use crate::models::user::User;
use crate::services::traits::EmailServiceBase;
use crate::utils::security::hash_password;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Error, PgPool};
use std::sync::Arc;
use tracing::info;
use tracing::log::error;
use uuid::Uuid;

pub struct Users {
    pool: PgPool,
    config: Arc<Config>,
}

impl Users {
    pub fn new(pool: PgPool, config: Arc<Config>) -> Self {
        Self { pool, config }
    }

    pub async fn create_user(&self, user_payload: RegisterUser) -> Result<User, UserError> {
        let user_id = Uuid::new_v4();
        let password_hash = match hash_password(&user_payload.password) {
            Ok(hash) => hash,
            Err(_) => return Err(UserError::InternalServerError),
        };
        let email_verified = false;
        match sqlx::query(
            r#"
            INSERT INTO users (id, username, email, password_hash, email_verified)
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(user_id)
        .bind(user_payload.username.clone())
        .bind(user_payload.email.clone())
        .bind(password_hash.clone())
        .bind(email_verified)
        .execute(&self.pool)
        .await
        {
            Ok(_) => Ok(()),
            Err(e) => {
                if let Error::Database(db_err) = &e {
                    if db_err.constraint() == Some("users_username_key")
                        || db_err.constraint() == Some("users_email_key")
                    {
                        return Err(UserError::AccountAlreadyExists);
                    }
                }
                error!("Failed to save users: {}", e);
                return Err(UserError::InternalServerError);
            }
        }?;
        Ok(User {
            id: user_id,
            email: user_payload.email,
            password_hash,
            username: user_payload.username,
            email_verified,
            created_at: Default::default(),
            updated_at: Default::default(),
        })
    }

    pub async fn get_user_by_email_or_username(&self, identity: &str) -> Result<User, UserError> {
        info!("Querying user: {}", identity);
        let user_result =
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1 OR email = $1")
                .bind(identity)
                .fetch_optional(&self.pool)
                .await;

        match user_result {
            Ok(Some(user)) => Ok(user),
            Ok(None) => {
                info!("User not found for {}", identity);
                Err(UserError::UserNotFound("Invalid credentials".to_string()))
            }
            Err(err) => {
                error!("Failed to querying user: {}", err.to_string());
                Err(UserError::InternalServerError)
            }
        }
    }
}
