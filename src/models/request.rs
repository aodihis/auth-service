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

#[derive(Deserialize, Debug, Validate)]
pub struct Token {
    pub token: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn test_valid_user_input() {
        let user = RegisterUser {
            email: "test@example.com".into(),
            username: "user123".into(),
            password: "Valid1@pass".into(),
        };

        let result = user.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_email() {
        let user = RegisterUser {
            email: "invalid_email".into(),
            username: "user123".into(),
            password: "Valid1@pass".into(),
        };

        let result = user.validate();
        assert!(result.is_err());
        let binding = result.unwrap_err();
        let errors = binding.field_errors();
        assert!(errors.contains_key("email"));
    }

    #[test]
    fn test_short_username() {
        let user = RegisterUser {
            email: "test@example.com".into(),
            username: "us".into(),
            password: "Valid1@pass".into(),
        };

        let result = user.validate();
        assert!(result.is_err());
        let binding = result.unwrap_err();
        let errors = binding.field_errors();
        assert!(errors.contains_key("username"));
    }

    #[test]
    fn test_password_too_short() {
        let result = validate_password("A1@bc");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code, "password_too_short");
    }

    #[test]
    fn test_password_no_lowercase() {
        let result = validate_password("PASSWORD1@");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code, "password_no_lowercase");
    }

    #[test]
    fn test_password_no_uppercase() {
        let result = validate_password("password1@");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code, "password_no_uppercase");
    }

    #[test]
    fn test_password_no_number() {
        let result = validate_password("Password@");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code, "password_no_number");
    }

    #[test]
    fn test_password_no_special_char() {
        let result = validate_password("Password1");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code, "password_no_special_char");
    }
}
