use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub full_name: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug)]
pub struct NewUser {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub full_name: String,
    pub created_at: String,
    pub updated_at: String,
}
