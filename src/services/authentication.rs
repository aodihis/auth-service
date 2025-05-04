use std::sync::Arc;
use chrono::{Duration, Utc};
use crate::config::Config;
use sqlx::{AnyPool};
use tracing::log::error;
use tracing_subscriber::fmt::format;
use uuid::Uuid;
use crate::error::authentication::RegisterError;
use crate::models::activation_token::ActivationToken;
use crate::models::request::RegisterUser;
use crate::services::traits::EmailServiceBase;
use crate::utils::security::hash_password;

pub struct Authentication {
    pool: AnyPool,
    email_service: Arc<dyn EmailServiceBase>,
    config: Arc<Config>
}

impl Authentication {
    pub fn new(pool: AnyPool, email_service: Arc<dyn EmailServiceBase>, config: Arc<Config>) -> Self {
        Self {
            pool,
            email_service,
            config
        }
    }

    pub async fn register(&self, payload: RegisterUser) -> Result<(), RegisterError> {


        let user_id = Uuid::new_v4();
        let password_hash = match hash_password(&payload.password) {
            Ok(hash) => hash,
            Err(_) => return Err(RegisterError::InternalServerError),
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
                        return Err(RegisterError::AccountAlreadyExists)
                    }
                }
                return Err(RegisterError::InternalServerError)
            }
        }?;

        let _ = self.send_activation_token(user_id);

        Ok(())

    }

    async fn send_activation_token(&self, user_id: Uuid) {
        let activation_token = ActivationToken {
            user_id,
            token: Uuid::new_v4().to_string(),
            expires_at: Utc::now() +  Duration::days(15),
        };

        let verify_url = format!("{}?token={}", self.config.app.verification_url, activation_token.token);
        self.save_activation_token(&activation_token).await;
        let template_string = format!(
            r#"
            Hello,

            Please click the link below to activate your account:
            urls?token={}

                Best regards,
                Your App Team
                "#,
                    verify_url,
        );

        let _ = self.email_service.send_email(
            "test@example.com".parse().unwrap(), vec![], vec![], "Account Activation".parse().unwrap(), template_string
        );
    }

    async fn save_activation_token(&self, activation_token: &ActivationToken) {
        match sqlx::query(
            r#"
            INSERT INTO verification_tokens (user_id, token, expires_at)
            VALUES ($1, $2, $3)
            "#,
        )
            .bind(activation_token.user_id.to_string())
            .bind(activation_token.token.clone())
            .bind(activation_token.expires_at.to_string())
            .execute(&self.pool)
            .await
        {
            Ok(_) => {}
            Err(e) => {
                error!("Failed to save activation token : {}", e)
            }
        };


    }
}

