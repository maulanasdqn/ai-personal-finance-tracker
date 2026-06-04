use super::dto::{AuthOutput, LoginInput};
use crate::domain::user::repository::UserRepository;
use crate::error::AppError;
use crate::infrastructure::auth::{jwt, password};

pub async fn execute(
    input: LoginInput,
    repo: &impl UserRepository,
    jwt_secret: &str,
) -> Result<AuthOutput, AppError> {
    let user = repo
        .find_by_email(&input.email)
        .await?
        .ok_or_else(|| AppError::Unauthorized("invalid email or password".into()))?;
    let valid = password::verify_password(&input.password, &user.password_hash)?;
    if !valid {
        return Err(AppError::Unauthorized("invalid email or password".into()));
    }
    let token = jwt::generate_token(&user.id, &user.email, jwt_secret)?;
    Ok(AuthOutput {
        token,
        user_id: user.id,
        email: user.email,
        full_name: user.full_name,
    })
}
