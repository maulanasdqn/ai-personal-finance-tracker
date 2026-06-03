pub struct RegisterInput {
    pub email: String,
    pub password: String,
    pub full_name: String,
}

pub struct LoginInput {
    pub email: String,
    pub password: String,
}

pub struct AuthOutput {
    pub token: String,
    pub user_id: String,
    pub email: String,
    pub full_name: String,
}
