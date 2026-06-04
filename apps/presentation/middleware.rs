use crate::error::AppError;
use crate::infrastructure::auth::jwt;
use worker::Request;

pub struct AuthUser {
    pub user_id: String,
}

pub fn extract_bearer(req: &Request) -> Result<String, AppError> {
    let header = req
        .headers()
        .get("authorization")
        .map_err(|_| AppError::Unauthorized("missing authorization header".into()))?
        .ok_or_else(|| AppError::Unauthorized("missing authorization header".into()))?;
    header
        .strip_prefix("Bearer ")
        .map(String::from)
        .ok_or_else(|| AppError::Unauthorized("invalid authorization format".into()))
}

pub fn authenticate(req: &Request, jwt_secret: &str) -> Result<AuthUser, AppError> {
    let token = extract_bearer(req)?;
    let claims = jwt::verify_token(&token, jwt_secret)?;
    Ok(AuthUser {
        user_id: claims.user_id,
    })
}
