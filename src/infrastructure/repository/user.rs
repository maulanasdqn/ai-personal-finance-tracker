use crate::domain::user::entity::{NewUser, User};
use crate::error::AppError;
use serde::Deserialize;
use worker::D1Database;

#[derive(Deserialize)]
struct UserRow {
    id: String,
    email: String,
    password_hash: String,
    full_name: String,
    created_at: String,
    updated_at: String,
}

impl From<UserRow> for User {
    fn from(r: UserRow) -> Self {
        Self {
            id: r.id,
            email: r.email,
            password_hash: r.password_hash,
            full_name: r.full_name,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

pub struct D1UserRepository {
    db: D1Database,
}

impl D1UserRepository {
    pub fn new(db: D1Database) -> Self {
        Self { db }
    }

    pub async fn create(&self, user: NewUser) -> Result<User, AppError> {
        self.db
            .prepare("INSERT INTO users (id, email, password_hash, full_name, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)")
            .bind(&[
                user.id.clone().into(),
                user.email.into(),
                user.password_hash.into(),
                user.full_name.into(),
                user.created_at.into(),
                user.updated_at.into(),
            ])
            .map_err(|_| AppError::Internal)?
            .run()
            .await
            .map_err(|_| AppError::Conflict("email already registered".into()))?;

        self.find_by_id(&user.id).await?.ok_or_else(|| AppError::Internal)
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<User>, AppError> {
        self.db
            .prepare("SELECT * FROM users WHERE id = ?1")
            .bind(&[id.into()])
            .map_err(|_| AppError::Internal)?
            .first::<UserRow>(None)
            .await
            .map_err(|_| AppError::Internal)
            .map(|r| r.map(Into::into))
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        self.db
            .prepare("SELECT * FROM users WHERE email = ?1")
            .bind(&[email.into()])
            .map_err(|_| AppError::Internal)?
            .first::<UserRow>(None)
            .await
            .map_err(|_| AppError::Internal)
            .map(|r| r.map(Into::into))
    }
}
