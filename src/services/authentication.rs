use crate::config::Config;
use crate::error::authentication::AuthenticationError;
use crate::models::authenticate::ActivationToken;
use crate::models::request::RegisterUser;
use crate::services::email::EmailService;
use crate::services::traits::EmailServiceBase;
use crate::utils::security::{hash_password, verify_password};
use chrono::{Duration, Utc};
use sqlx::{AnyPool, Error, PgPool, Row};
use std::sync::Arc;
use tracing::info;
use tracing::log::error;
use tracing_subscriber::fmt::format;
use uuid::Uuid;
use crate::models::claims::Claims;
use crate::models::user::User;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
pub struct Authentication {
    pool: PgPool,
    config: Arc<Config>,
}

impl Authentication {
    pub fn new(pool: PgPool, config: Arc<Config>) -> Self {
        Self { pool, config }
    }

    pub async fn send_activation_token(
        &self,
        email_service: &EmailService,
        user_id: Uuid,
    ) -> Result<(), AuthenticationError> {
        info!("Sending activation token for user {}", user_id);
        let activation_token = ActivationToken {
            user_id,
            token: Uuid::new_v4().to_string(),
            expires_at: Utc::now() + Duration::days(15),
        };

        let verify_url = format!(
            "{}?token={}",
            self.config.app.verification_url, activation_token.token
        );
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

        let _ = email_service
            .send_email(
                "test@example.com".parse().unwrap(),
                vec![],
                vec![],
                "Account Activation".parse().unwrap(),
                template_string,
            )
            .await;

        Ok(())
    }

    pub async fn verify_user(&self, token: String) -> Result<(), AuthenticationError> {
        let result = sqlx::query(
            r#"
                    SELECT * FROM verification_tokens
                    WHERE token = $1 AND expires_at > NOW()
                    "#,
        )
        .bind(token)
        .fetch_optional(&self.pool)
        .await;

        match result {
            Ok(Some(row)) => {
                let user_id: Uuid = match row.try_get("user_id") {
                    Ok(id) => id,
                    Err(_) => {
                        return Err(AuthenticationError::InternalServerError);
                    }
                };
                self.activate_user(user_id).await
            }
            Ok(None) => Err(AuthenticationError::InvalidToken),
            Err(e) => {
                error!("Failed to verify email: {}", e);
                Err(AuthenticationError::InternalServerError)
            }
        }
    }

    pub async fn resend_activation_token(
        &self,
        email_service: &EmailService,
        user_id: &Uuid,
    ) -> Result<(), AuthenticationError> {
        self.remove_old_activation_token(user_id).await;
        self.send_activation_token(email_service, user_id.clone())
            .await
    }

    pub async fn login(&self, user: User, password: String) -> Result<String, AuthenticationError> {
        if !verify_password(&password, &user.password_hash) {
            return Err(AuthenticationError::InternalServerError)
        }

        let token = self.create_token(&user)?;
        Ok(token)
    }

    fn create_token(&self, user: &User) -> Result<String, AuthenticationError> {
        let expiration = Utc::now().checked_add_signed(Duration::days(self.config.jwt.expiration))
            .expect("valid timestamp").timestamp() as usize;
        let claims = Claims {
            sub: user.username.clone(),
            exp: expiration,
            iat: Utc::now().timestamp() as usize,
        };

        match encode(&Header::default(), &claims, &EncodingKey::from_secret(&self.config.jwt.secret.as_bytes())) {
            Ok(token) => Ok(token),
            Err(_) => Err(AuthenticationError::InternalServerError),
        }
    }

    async fn remove_old_activation_token(&self, user_id: &Uuid) {
        let _ = sqlx::query(
            r#"
                    DELETE FROM verification_tokens
                    WHERE user_id = $1
                    "#,
        )
        .bind(user_id)
        .execute(&self.pool)
        .await;
    }

    async fn activate_user(&self, user_id: Uuid) -> Result<(), AuthenticationError> {
        let res = sqlx::query!(
            r#"
                    UPDATE users
                    SET is_active = true
                    WHERE id = $1
                    "#,
            user_id
        )
        .execute(&self.pool)
        .await;
        match res {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Failed to activate user: {}", e);
                Err(AuthenticationError::InternalServerError)
            }
        }
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
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Failed to save activation token : {}", e);
                Err(e)
            }
        }
    }
}
