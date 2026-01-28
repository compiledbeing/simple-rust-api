use argon2::{
    Argon2, 
    PasswordHash, 
    PasswordHasher, 
    PasswordVerifier, 
    password_hash::{SaltString, rand_core::OsRng}
};

use crate::error::ApiError;

pub fn hash_password(password: &str) -> Result<String, ApiError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| ApiError::BadRequest)?
        .to_string();

    Ok(hash)
}

pub fn verify_password(hash: &str, password: &str) -> Result<(), ApiError> {
    let parsed = PasswordHash::new(hash).map_err(|_| ApiError::BadRequest)?;
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .map_err(|_| ApiError::BadRequest)
}
