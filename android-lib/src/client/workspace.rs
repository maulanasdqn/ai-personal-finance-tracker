use serde_json::json;
use super::{bearer, err_body};

pub fn list(base: &str, token: &str) -> Result<String, String> {
    ureq::get(&format!("{}/api/v1/workspaces", base))
        .set("Authorization", &bearer(token))
        .call()
        .map_err(err_body)?
        .into_string()
        .map_err(|e| e.to_string())
}

pub fn create(base: &str, token: &str, name: &str, description: Option<&str>) -> Result<String, String> {
    let mut body = json!({ "name": name });
    if let Some(d) = description {
        body["description"] = json!(d);
    }
    ureq::post(&format!("{}/api/v1/workspaces", base))
        .set("Authorization", &bearer(token))
        .send_json(body)
        .map_err(err_body)?
        .into_string()
        .map_err(|e| e.to_string())
}

pub fn get(base: &str, token: &str, id: &str) -> Result<String, String> {
    ureq::get(&format!("{}/api/v1/workspaces/{}", base, id))
        .set("Authorization", &bearer(token))
        .call()
        .map_err(err_body)?
        .into_string()
        .map_err(|e| e.to_string())
}

pub fn update(base: &str, token: &str, id: &str, name: Option<&str>, description: Option<&str>) -> Result<String, String> {
    let mut body = json!({});
    if let Some(n) = name { body["name"] = json!(n); }
    if let Some(d) = description { body["description"] = json!(d); }
    ureq::put(&format!("{}/api/v1/workspaces/{}", base, id))
        .set("Authorization", &bearer(token))
        .send_json(body)
        .map_err(err_body)?
        .into_string()
        .map_err(|e| e.to_string())
}

pub fn delete(base: &str, token: &str, id: &str) -> Result<String, String> {
    ureq::delete(&format!("{}/api/v1/workspaces/{}", base, id))
        .set("Authorization", &bearer(token))
        .call()
        .map_err(err_body)?
        .into_string()
        .map_err(|e| e.to_string())
}

pub fn list_members(base: &str, token: &str, id: &str) -> Result<String, String> {
    ureq::get(&format!("{}/api/v1/workspaces/{}/members", base, id))
        .set("Authorization", &bearer(token))
        .call()
        .map_err(err_body)?
        .into_string()
        .map_err(|e| e.to_string())
}

pub fn add_member(base: &str, token: &str, id: &str, email: &str) -> Result<String, String> {
    ureq::post(&format!("{}/api/v1/workspaces/{}/members", base, id))
        .set("Authorization", &bearer(token))
        .send_json(json!({ "email": email }))
        .map_err(err_body)?
        .into_string()
        .map_err(|e| e.to_string())
}
