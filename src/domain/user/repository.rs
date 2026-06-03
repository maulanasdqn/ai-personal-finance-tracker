use crate::domain::user::entity::{NewUser, User};
use crate::error::AppError;
use std::future::Future;

pub trait UserRepository: Send + Sync {
    fn create(&self, user: NewUser) -> impl Future<Output = Result<User, AppError>> + Send;
    fn find_by_id(&self, id: &str) -> impl Future<Output = Result<Option<User>, AppError>> + Send;
    fn find_by_email(&self, email: &str) -> impl Future<Output = Result<Option<User>, AppError>> + Send;
}
