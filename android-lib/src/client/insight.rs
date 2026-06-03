use serde_json::json;
use super::{bearer, err_body};

pub fn list(base: &str, token: &str, workspace_id: &str, insight_type: Option<&str>) -> Result<String, String> {
    let mut url = format!("{}/api/v1/workspaces/{}/insights?", base, workspace_id);
    if let Some(t) = insight_type { url.push_str(&format!("type={}&", t)); }
    ureq::get(&url)
        .set("Authorization", &bearer(token))
        .call()
        .map_err(err_body)?
        .into_string()
        .map_err(|e| e.to_string())
}

pub fn generate(base: &str, token: &str, workspace_id: &str, date_from: &str, date_to: &str) -> Result<String, String> {
    ureq::post(&format!("{}/api/v1/workspaces/{}/insights/generate", base, workspace_id))
        .set("Authorization", &bearer(token))
        .send_json(json!({ "date_from": date_from, "date_to": date_to }))
        .map_err(err_body)?
        .into_string()
        .map_err(|e| e.to_string())
}
