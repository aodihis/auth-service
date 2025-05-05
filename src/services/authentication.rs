use std::sync::Arc;
use chrono::{Duration, Utc};
use crate::config::Config;
use sqlx::{AnyPool, Error, PgPool};
use tracing::info;
use tracing::log::error;
use tracing_subscriber::fmt::format;
use uuid::Uuid;
use crate::error::authentication::AuthenticationError;
use crate::models::activation_token::ActivationToken;
use crate::models::request::RegisterUser;
use crate::services::traits::EmailServiceBase;
use crate::utils::security::hash_password;

pub struct Authentication {
    pool: PgPool,
    email_service: Arc<dyn EmailServiceBase>,
    config: Arc<Config>
}

impl Authentication {
    pub fn new(pool: PgPool, email_service: Arc<dyn EmailServiceBase>, config: Arc<Config>) -> Self {
        Self {
            pool,
            email_service,
            config
        }
    }

    pub async fn register(&self, payload: RegisterUser) -> Result<(), AuthenticationError> {

        let user_id = Uuid::new_v4();
        let password_hash = match hash_password(&payload.password) {
            Ok(hash) => hash,
            Err(_) => return Err(AuthenticationError::InternalServerError),
        };

        match sqlx::query(
            r#"
            INSERT INTO users (id, username, email, password_hash)
            VALUES ($1, $2, $3, $4)
            "#,
        )
            .bind(user_id)
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
                        return Err(AuthenticationError::AccountAlreadyExists)
                    }
                }
                error!("Failed to save users: {}", e.to_string());
                return Err(AuthenticationError::InternalServerError)
            }
        }?;

        let _ = self.send_activation_token(user_id).await;
        Ok(())

    }

    async fn send_activation_token(&self, user_id: Uuid) -> Result<(), AuthenticationError> {
        info!("Sending activation token for user {}", user_id);
        let activation_token = ActivationToken {
            user_id,
            token: Uuid::new_v4().to_string(),
            expires_at: Utc::now() +  Duration::days(15),
        };

        let verify_url = format!("{}?token={}", self.config.app.verification_url, activation_token.token);
        match self.save_activation_token(&activation_token).await {
            Ok(_) => {}
            Err(_) => {
                return Err(AuthenticationError::InternalServerError);
            }
        };
        let template_string = format!(
            r#"
            Hello,

            Please click the link below to activate your account:
            <a href={}>activate</a>

                Best regards,
                Your App Team
                "#,
                    verify_url,
        );

        let res = self.email_service.send_email(
            "test@example.com".parse().unwrap(), vec![], vec![], "Account Activation".parse().unwrap(), template_string
        ).await;

        Ok(())
    }

    async fn save_activation_token(&self, activation_token: &ActivationToken) -> Result<(), Error> {
        match sqlx::query(
            r#"
            INSERT INTO verification_tokens (user_id, token, expires_at)
            VALUES ($1, $2, $3)
            "#,
        )
            .bind(activation_token.user_id)
            .bind(activation_token.token.clone())
            .bind(activation_token.expires_at)
            .execute(&self.pool)
            .await
        {
            Ok(_) => {
                Ok(())
            }
            Err(e) => {
                error!("Failed to save activation token : {}", e);
                Err(e)
            }
        }
    }
}

