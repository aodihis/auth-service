use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use tracing::{debug, error};

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    Ok(password_hash)
}

pub fn verify_password(password: &str, hashed_password: &str) -> bool {
    debug!("Parsing password");
    let parsed_hash = match PasswordHash::new(hashed_password) {
        Ok(h) => h,
        Err(err) => {
            error!("Failed to parse hashed password: {}", err);
            return false;
        }
    };
    debug!("Verifying password");
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}
