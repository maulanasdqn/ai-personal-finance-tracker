use serde_json::json;
use super::err_body;

pub fn register(base: &str, email: &str, password: &str, full_name: &str) -> Result<String, String> {
    ureq::post(&format!("{}/api/v1/auth/register", base))
        .send_json(json!({ "email": email, "password": password, "full_name": full_name }))
        .map_err(err_body)?
        .into_string()
        .map_err(|e| e.to_string())
}

pub fn login(base: &str, email: &str, password: &str) -> Result<String, String> {
    ureq::post(&format!("{}/api/v1/auth/login", base))
        .send_json(json!({ "email": email, "password": password }))
        .map_err(err_body)?
        .into_string()
        .map_err(|e| e.to_string())
}
