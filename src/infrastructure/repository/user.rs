use crate::domain::user::entity::{NewUser, User};
use crate::domain::user::repository::UserRepository;
use crate::error::AppError;
use rust_d1_orm::{D1Model, Query, Table};
use serde::Deserialize;
use worker::{wasm_bindgen::JsValue, D1Database};

#[derive(Deserialize)]
struct UserRow {
    id: String,
    email: String,
    password_hash: String,
    full_name: String,
    created_at: String,
    updated_at: String,
}

impl D1Model for UserRow {
    const TABLE: &'static str = "users";
    const COLUMNS: &'static [&'static str] = &["id", "email", "password_hash", "full_name", "created_at", "updated_at"];
    fn values(&self) -> Vec<JsValue> {
        vec![
            self.id.clone().into(), self.email.clone().into(),
            self.password_hash.clone().into(), self.full_name.clone().into(),
            self.created_at.clone().into(), self.updated_at.clone().into(),
        ]
    }
}

impl From<UserRow> for User {
    fn from(r: UserRow) -> Self {
        Self { id: r.id, email: r.email, password_hash: r.password_hash, full_name: r.full_name, created_at: r.created_at, updated_at: r.updated_at }
    }
}

pub struct D1UserRepository { db: D1Database }

impl D1UserRepository {
    pub fn new(db: D1Database) -> Self { Self { db } }
}

impl UserRepository for D1UserRepository {
    async fn create(&self, user: NewUser) -> Result<User, AppError> {
        let row = UserRow {
            id: user.id, email: user.email, password_hash: user.password_hash,
            full_name: user.full_name, created_at: user.created_at, updated_at: user.updated_at,
        };
        Table::<UserRow>::new(&self.db)
            .insert(&row).await
            .map_err(|_| AppError::Conflict("email already registered".into()))
            .map(Into::into)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        Table::<UserRow>::new(&self.db)
            .find_one(Query::new().eq("email", email)).await
            .map_err(|_| AppError::Internal)
            .map(|r| r.map(Into::into))
    }
}
