use super::{bearer, err_body};

pub fn list(base: &str, token: &str, workspace_id: &str) -> Result<String, String> {
    ureq::get(&format!("{}/api/v1/workspaces/{}/statements", base, workspace_id))
        .set("Authorization", &bearer(token))
        .call()
        .map_err(err_body)?
        .into_string()
        .map_err(|e| e.to_string())
}

pub fn get(base: &str, token: &str, workspace_id: &str, id: &str) -> Result<String, String> {
    ureq::get(&format!("{}/api/v1/workspaces/{}/statements/{}", base, workspace_id, id))
        .set("Authorization", &bearer(token))
        .call()
        .map_err(err_body)?
        .into_string()
        .map_err(|e| e.to_string())
}

pub fn upload(base: &str, token: &str, workspace_id: &str, file_name: &str, content_type: &str, data: &[u8]) -> Result<String, String> {
    ureq::post(&format!("{}/api/v1/workspaces/{}/statements", base, workspace_id))
        .set("Authorization", &bearer(token))
        .set("Content-Type", content_type)
        .set("X-File-Name", file_name)
        .send_bytes(data)
        .map_err(err_body)?
        .into_string()
        .map_err(|e| e.to_string())
}
