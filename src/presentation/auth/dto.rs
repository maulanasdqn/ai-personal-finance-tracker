use serde::{Deserialize, Serialize};
use zod_rs::prelude::*;

#[derive(Deserialize, ZodSchema)]
pub struct RegisterRequest {
    #[zod(min_length(1), max_length(254), email)]
    pub email: String,
    #[zod(min_length(8), max_length(128))]
    pub password: String,
    #[zod(min_length(1), max_length(100))]
    pub full_name: String,
}

#[derive(Deserialize, ZodSchema)]
pub struct LoginRequest {
    #[zod(min_length(1), max_length(254), email)]
    pub email: String,
    #[zod(min_length(1))]
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: String,
    pub email: String,
    pub full_name: String,
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_node);
    use super::*;

    #[wasm_bindgen_test]
    fn register_valid_input() {
        let v = serde_json::json!({
            "email": "user@example.com",
            "password": "Password1",
            "full_name": "Alice"
        });
        assert!(RegisterRequest::validate_and_parse(&v).is_ok());
    }

    #[wasm_bindgen_test]
    fn register_missing_email() {
        let v = serde_json::json!({
            "password": "Password1",
            "full_name": "Alice"
        });
        assert!(RegisterRequest::validate_and_parse(&v).is_err());
    }

    #[wasm_bindgen_test]
    fn register_invalid_email_format() {
        let v = serde_json::json!({
            "email": "not-an-email",
            "password": "Password1",
            "full_name": "Alice"
        });
        assert!(RegisterRequest::validate_and_parse(&v).is_err());
    }

    #[wasm_bindgen_test]
    fn register_password_too_short() {
        let v = serde_json::json!({
            "email": "user@example.com",
            "password": "Pwd1",
            "full_name": "Alice"
        });
        assert!(RegisterRequest::validate_and_parse(&v).is_err());
    }

    #[wasm_bindgen_test]
    fn register_empty_full_name() {
        let v = serde_json::json!({
            "email": "user@example.com",
            "password": "Password1",
            "full_name": ""
        });
        assert!(RegisterRequest::validate_and_parse(&v).is_err());
    }

    #[wasm_bindgen_test]
    fn login_valid_input() {
        let v = serde_json::json!({
            "email": "user@example.com",
            "password": "anypassword"
        });
        assert!(LoginRequest::validate_and_parse(&v).is_ok());
    }

    #[wasm_bindgen_test]
    fn login_empty_password() {
        let v = serde_json::json!({
            "email": "user@example.com",
            "password": ""
        });
        assert!(LoginRequest::validate_and_parse(&v).is_err());
    }
}
