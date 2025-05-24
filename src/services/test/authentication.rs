use crate::config::load_config;
use crate::error::authentication::AuthenticationError;
use crate::error::email::EmailError;
use crate::models::claims::Claims;
use crate::models::request::RegisterUser;
use crate::services::authentications::Authentication;
use crate::services::traits::EmailServiceBase;
use crate::services::users::Users;
use anyhow::Error;
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use sqlx::PgPool;
use std::pin::Pin;
use std::sync::Arc;

#[sqlx::test]
async fn test_verify_email(pool: PgPool) -> Result<(), Error> {
    use chrono::{Duration, Utc};
    use sqlx::query;
    use std::sync::Arc;

    // Setup
    let config = Arc::new(load_config()?);
    let user_service = Users::new(pool.clone(), config.clone());
    let authentication_service = Authentication::new(pool.clone(), config);

    // Create a dummy user to associate with the token
    let test_user = RegisterUser {
        username: "verifyuser".to_string(),
        email: "verify@example.com".to_string(),
        password: "VerifyPassword123!".to_string(),
    };
    let created_user = user_service.create_user(test_user.clone()).await?;

    // Create a valid token
    let token = "valid-test-token";
    let expires_at = Utc::now() + Duration::minutes(10);
    query(
        r#"
        INSERT INTO verification_tokens (token, user_id, expires_at)
        VALUES ($1, $2, $3)
        "#,
    )
    .bind(token)
    .bind(created_user.id)
    .bind(expires_at)
    .execute(&pool)
    .await?;

    // Test: valid token should pass
    let result = authentication_service.verify_email(token.to_string()).await;
    assert!(result.is_ok(), "Expected token verification to succeed");

    // Test: invalid token
    let invalid_result = authentication_service
        .verify_email("nonexistent-token".to_string())
        .await;
    assert!(
        matches!(invalid_result, Err(AuthenticationError::InvalidToken)),
        "Expected InvalidToken error"
    );

    // Test: expired token
    let expired_token = "expired-token";
    let expired_time = Utc::now() - Duration::minutes(10);
    query(
        r#"
        INSERT INTO verification_tokens (token, user_id, expires_at)
        VALUES ($1, $2, $3)
        "#,
    )
    .bind(expired_token)
    .bind(created_user.id)
    .bind(expired_time)
    .execute(&pool)
    .await?;

    let expired_result = authentication_service
        .verify_email(expired_token.to_string())
        .await;
    assert!(
        matches!(expired_result, Err(AuthenticationError::InvalidToken)),
        "Expected InvalidToken for expired token"
    );

    Ok(())
}

#[sqlx::test]
async fn test_login_success_and_invalid_password(pool: PgPool) -> Result<(), Error> {
    use std::sync::Arc;

    // Setup
    let config = Arc::new(load_config()?);
    let user_service = Users::new(pool.clone(), config.clone());
    let authentication_service = Authentication::new(pool.clone(), config);

    let test_user = RegisterUser {
        username: "loginuser".to_string(),
        email: "login@example.com".to_string(),
        password: "StrongPassword123!".to_string(),
    };
    let created_user = user_service.create_user(test_user.clone()).await?;

    // Correct password
    let token_result = authentication_service
        .login(created_user.clone(), test_user.password.clone())
        .await;
    assert!(
        token_result.is_ok(),
        "Expected login with correct password to succeed"
    );
    let token = token_result?;

    // Incorrect password
    let bad_password_result = authentication_service
        .login(created_user, "WrongPassword!".to_string())
        .await;
    assert!(
        matches!(
            bad_password_result,
            Err(AuthenticationError::InvalidCredentials)
        ),
        "Expected InvalidCredentials error"
    );

    Ok(())
}

#[sqlx::test]
async fn test_validate_token(pool: PgPool) -> Result<(), Error> {
    use std::sync::Arc;

    // Setup
    let config = Arc::new(load_config()?);
    let jwt = config.jwt.secret.clone();
    let user_service = Users::new(pool.clone(), config.clone());
    let authentication_service = Authentication::new(pool.clone(), config);

    // Create test user and token
    let test_user = RegisterUser {
        username: "tokenuser".to_string(),
        email: "token@example.com".to_string(),
        password: "TokenPassword123!".to_string(),
    };

    let created_user = user_service.create_user(test_user.clone()).await?;

    let expiration = Utc::now()
        .checked_add_signed(Duration::days(30))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: test_user.username.clone(),
        exp: expiration,
        iat: Utc::now().timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt.as_bytes()),
    )?;

    // Validate correct token
    let validated = authentication_service.validate_token(token.clone());
    assert!(validated.is_ok(), "Expected valid token to pass");
    assert_eq!(validated?, created_user.username);

    // Validate invalid token
    let bad_token = "invalid.token.value".to_string();
    let bad_result = authentication_service.validate_token(bad_token);
    assert!(
        matches!(bad_result, Err(AuthenticationError::InvalidToken)),
        "Expected InvalidToken error"
    );

    Ok(())
}

struct EmailService {}

impl EmailServiceBase for EmailService {
    fn send_email(
        &self,
        _: String,
        _: Vec<String>,
        _: Vec<String>,
        _: String,
        _: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), EmailError>> + Send>> {
        Box::pin(async move { Ok(()) })
    }
}
#[sqlx::test]
async fn test_send_activation_token(pool: PgPool) -> Result<(), Error> {
    // Setup
    let config = Arc::new(load_config()?);
    let user_service = Users::new(pool.clone(), config.clone());
    let authentication_service = Authentication::new(pool.clone(), config);
    let email_service = EmailService {};

    let test_user = RegisterUser {
        username: "verifyuser".to_string(),
        email: "verify@example.com".to_string(),
        password: "VerifyPassword123!".to_string(),
    };
    let created_user = user_service.create_user(test_user.clone()).await?;

    let res = authentication_service
        .send_activation_token(&email_service, created_user.id)
        .await;
    assert!(res.is_ok(), "Expected activation token to succeed");

    let result = sqlx::query(
        r#"
                    SELECT * FROM verification_tokens
                    WHERE user_id = $1
                    "#,
    )
    .bind(created_user.id)
    .fetch_optional(&pool)
    .await;

    assert!(result.is_ok());
    let result = result?;

    assert!(result.is_some());
    Ok(())
}

#[sqlx::test]
async fn test_resend_activation_token(pool: PgPool) -> Result<(), Error> {
    // Setup
    let config = Arc::new(load_config()?);
    let user_service = Users::new(pool.clone(), config.clone());
    let authentication_service = Authentication::new(pool.clone(), config);
    let email_service = EmailService {};

    let test_user = RegisterUser {
        username: "verifyuser".to_string(),
        email: "verify@example.com".to_string(),
        password: "VerifyPassword123!".to_string(),
    };
    let created_user = user_service.create_user(test_user.clone()).await?;

    authentication_service
        .send_activation_token(&email_service, created_user.id)
        .await?;

    let res = authentication_service
        .resend_activation_token(&email_service, &created_user.id)
        .await;

    assert!(res.is_ok(), "Expected send token to succeed");
    Ok(())
}
