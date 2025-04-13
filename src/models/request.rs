use serde::Deserialize;
use validator::{Validate, ValidationError};

#[derive(Deserialize, Debug, Validate)]
pub struct RegisterUser {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 3, message = "Username must be at least 3 characters"))]
    pub username: String,
    #[validate(custom(function="validate_password"))]
    pub password: String,
}

fn validate_password(password: &str) -> Result<(), ValidationError> {
    if password.len() < 8 {
        return Err(ValidationError::new("password_too_short"));
    }

    if !password.chars().any(|c| c.is_lowercase()) {
        return Err(ValidationError::new("password_no_lowercase"));
    }

    if !password.chars().any(|c| c.is_uppercase()) {
        return Err(ValidationError::new("password_no_uppercase"));
    }

    if !password.chars().any(|c| c.is_numeric()) {
        return Err(ValidationError::new("password_no_number"));
    }

    // Optional: check for special characters
    if !password.chars().any(|c| !c.is_alphanumeric()) {
        return Err(ValidationError::new("password_no_special_char"));
    }

    Ok(())
}
