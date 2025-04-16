use sqlx::{AnyPool};
use uuid::Uuid;
use crate::error::RegisterError;
use crate::error::RegisterError::{AccountAlreadyExists, InternalServerError};
use crate::models::request::RegisterUser;
use crate::utils::security::hash_password;

pub struct Authentication {
    pool: AnyPool,
}

impl Authentication {
    pub fn new(pool: AnyPool) -> Self {
        Self {
            pool
        }
    }

    pub async fn register(&self, payload: RegisterUser) -> Result<(), RegisterError> {


        let user_id = Uuid::new_v4();
        let password_hash = match hash_password(&payload.password) {
            Ok(hash) => hash,
            Err(_) => return Err(InternalServerError),
        };

        match sqlx::query(
            r#"
            INSERT INTO users (id, username, email, password_hash)
            VALUES ($1, $2, $3, $4)
            "#,
        )
            .bind(user_id.to_string())
            .bind(payload.username)
            .bind(payload.email)
            .bind(password_hash)
            .execute(&self.pool)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => {
                if let sqlx::Error::Database(db_err) = &e {
                    if db_err.constraint() == Some("users_username_key") ||
                        db_err.constraint() == Some("users_email_key") {
                        return Err(AccountAlreadyExists)
                    }
                }
                Err(InternalServerError)
            }
        }

    }

}

