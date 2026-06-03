use crate::error::AppError;
use jwt_simple::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AuthClaims {
    pub user_id: String,
    pub email: String,
}

pub fn generate_token(user_id: &str, email: &str, secret: &str) -> Result<String, AppError> {
    let key = HS256Key::from_bytes(secret.as_bytes());
    let claims = Claims::with_custom_claims(
        AuthClaims { user_id: user_id.to_string(), email: email.to_string() },
        Duration::from_hours(24),
    );
    key.authenticate(claims).map_err(|_| AppError::Internal)
}

pub fn verify_token(token: &str, secret: &str) -> Result<AuthClaims, AppError> {
    let key = HS256Key::from_bytes(secret.as_bytes());
    let claims = key
        .verify_token::<AuthClaims>(token, None)
        .map_err(|_| AppError::Unauthorized("invalid or expired token".into()))?;
    Ok(claims.custom)
}
