use crate::error::AppError;
use pbkdf2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Pbkdf2,
};
use getrandom::getrandom;

pub fn hash_password(password: &str) -> Result<String, AppError> {
    let mut salt_bytes = [0u8; 16];
    getrandom(&mut salt_bytes).map_err(|_| AppError::Internal)?;
    let salt = SaltString::encode_b64(&salt_bytes)
        .map_err(|_| AppError::Internal)?;
    Pbkdf2
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|_| AppError::Internal)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    let parsed = PasswordHash::new(hash).map_err(|_| AppError::Internal)?;
    Ok(Pbkdf2.verify_password(password.as_bytes(), &parsed).is_ok())
}
