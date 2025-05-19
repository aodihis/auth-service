use crate::config::load_config;
use crate::error::user::UserError;
use crate::models::request::RegisterUser;
use crate::models::user::User;
use crate::services::users::Users;
use crate::utils::security::verify_password;
use anyhow::Error;
use sqlx::PgPool;
use std::sync::Arc;

#[sqlx::test()]
async fn test_create_user_success(pool: PgPool) -> Result<(), Error> {
    // Setup
    let config = Arc::new(load_config()?);
    let user_service = Users::new(pool.clone(), config);

    // Create test data
    let test_user = RegisterUser {
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password: "Password123!".to_string(),
    };

    // Execute
    let result = user_service.create_user(test_user.clone()).await;

    // Assert
    assert!(result.is_ok(), "Expected user creation to succeed");
    let created_user = result?;
    assert_eq!(created_user.email, test_user.email);
    assert_ne!(created_user.password_hash, test_user.password); // Password should be hashed

    // Verify the user exists in the database
    let db_user =
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1 OR email = $1")
            .bind(test_user.email.clone())
            .fetch_one(&pool)
            .await?;

    assert_eq!(db_user.id, created_user.id);
    assert_eq!(db_user.email, test_user.email);
    assert_eq!(db_user.email_verified, false);

    Ok(())
}

#[sqlx::test()]
async fn test_create_user_duplicate_email(pool: PgPool) -> Result<(), Error> {
    // Setup
    let config = Arc::new(load_config()?);
    let user_service = Users::new(pool.clone(), config);

    // Create first user
    let first_user = RegisterUser {
        username: "user1".to_string(),
        email: "duplicate@example.com".to_string(),
        password: "Password123!".to_string(),
    };
    user_service.create_user(first_user).await?;

    // Try to create second user with same email
    let second_user = RegisterUser {
        username: "user2".to_string(),
        email: "duplicate@example.com".to_string(), // Same email
        password: "AnotherPassword456!".to_string(),
    };

    // Execute
    let result = user_service.create_user(second_user).await;

    // Assert
    assert!(result.is_err());
    if let Err(err) = result {
        assert!(
            matches!(err, UserError::AccountAlreadyExists),
            "Expected AccountAlreadyExists error, got: {:?}",
            err
        );
    }

    // Verify only one user exists with this email
    let count = sqlx::query!(
        r#"SELECT COUNT(*) as count FROM users WHERE email = $1"#,
        "duplicate@example.com"
    )
    .fetch_one(&pool)
    .await?
    .count
    .unwrap_or(0);

    assert_eq!(count, 1, "Expected only one user with this email");

    Ok(())
}

#[sqlx::test()]
async fn test_create_user_duplicate_username(pool: PgPool) -> Result<(), Error> {
    // Setup
    let config = Arc::new(load_config()?);
    let user_service = Users::new(pool.clone(), config);

    // Create first user
    let first_user = RegisterUser {
        username: "sameusername".to_string(),
        email: "user1@example.com".to_string(),
        password: "Password123!".to_string(),
    };
    user_service.create_user(first_user).await?;

    // Try to create second user with same username
    let second_user = RegisterUser {
        username: "sameusername".to_string(), // Same username
        email: "user2@example.com".to_string(),
        password: "AnotherPassword456!".to_string(),
    };

    // Execute
    let result = user_service.create_user(second_user).await;

    // Assert
    assert!(result.is_err());
    if let Err(err) = result {
        assert!(
            matches!(err, UserError::AccountAlreadyExists),
            "Expected AccountAlreadyExists error, got: {:?}",
            err
        );
    }

    Ok(())
}

// This test verifies that the password is properly hashed
#[sqlx::test()]
async fn test_password_is_properly_hashed(pool: PgPool) -> Result<(), Error> {
    // Setup
    let config = Arc::new(load_config()?);
    let user_service = Users::new(pool.clone(), config);

    // Create test data
    let password = "SecurePassword123!";
    let test_user = RegisterUser {
        username: "securityuser".to_string(),
        email: "security@example.com".to_string(),
        password: password.to_string(),
    };

    // Execute
    let created_user = user_service.create_user(test_user.clone()).await?;

    // Assert
    assert_ne!(
        created_user.password_hash, password,
        "Password should be hashed"
    );
    assert!(
        !created_user.password_hash.is_empty(),
        "Password hash should not be empty"
    );

    assert_eq!(created_user.username, test_user.username);
    assert_eq!(created_user.email, test_user.email);

    let res = verify_password(&password, &created_user.password_hash);
    assert!(res);
    Ok(())
}

#[sqlx::test]
async fn test_get_user_by_email_or_username(pool: PgPool) -> Result<(), Error> {
    use std::sync::Arc;

    // Setup
    let config = Arc::new(load_config()?);
    let user_service = Users::new(pool.clone(), config);

    // Create test data
    let test_user = RegisterUser {
        username: "testuser".to_string(),
        email: "testuser@example.com".to_string(),
        password: "TestPassword123!".to_string(),
    };

    // Create user in DB
    let _ = user_service.create_user(test_user.clone()).await?;

    // Test by email
    let result_by_email = user_service
        .get_user_by_email_or_username(&test_user.email)
        .await;
    assert!(
        result_by_email.is_ok(),
        "Expected user lookup by email to succeed"
    );
    assert_eq!(result_by_email?.email, test_user.email);

    // Test by username
    let result_by_username = user_service
        .get_user_by_email_or_username(&test_user.username)
        .await;
    assert!(
        result_by_username.is_ok(),
        "Expected user lookup by username to succeed"
    );
    assert_eq!(result_by_username?.username, test_user.username);

    // Test for non-existent user
    let result_invalid = user_service
        .get_user_by_email_or_username("nonexistent")
        .await;
    assert!(
        result_invalid.is_err(),
        "Expected lookup to fail for nonexistent user"
    );
    if let Err(UserError::UserNotFound(_)) = result_invalid {
        // Expected error
    } else {
        panic!("Expected UserNotFound error");
    }

    Ok(())
}
