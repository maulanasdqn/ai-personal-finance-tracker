use super::dto::{AuthOutput, RegisterInput};
use crate::domain::user::entity::NewUser;
use crate::error::AppError;
use crate::infrastructure::auth::{jwt, password};
use crate::infrastructure::repository::user::D1UserRepository;
use uuid::Uuid;

pub async fn execute(input: RegisterInput, repo: &D1UserRepository, jwt_secret: &str) -> Result<AuthOutput, AppError> {
    if repo.find_by_email(&input.email).await?.is_some() {
        return Err(AppError::Conflict("email already registered".into()));
    }
    let now = chrono::Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();
    let hash = password::hash_password(&input.password)?;
    let user = repo.create(NewUser { id: id.clone(), email: input.email.clone(), password_hash: hash, full_name: input.full_name.clone(), created_at: now.clone(), updated_at: now }).await?;
    let token = jwt::generate_token(&user.id, &user.email, jwt_secret)?;
    Ok(AuthOutput { token, user_id: user.id, email: user.email, full_name: user.full_name })
}
