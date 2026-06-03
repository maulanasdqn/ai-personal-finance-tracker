use crate::domain::user::entity::{NewUser, User};
use crate::error::AppError;

pub trait UserRepository {
    async fn create(&self, user: NewUser) -> Result<User, AppError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError>;
}
