use serde_json::json;
use super::{bearer, err_body};

pub struct TransactionFilter<'a> {
    pub category: Option<&'a str>,
    pub transaction_type: Option<&'a str>,
    pub from: Option<&'a str>,
    pub to: Option<&'a str>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

pub fn list(base: &str, token: &str, workspace_id: &str, f: TransactionFilter) -> Result<String, String> {
    let mut url = format!("{}/api/v1/workspaces/{}/transactions?", base, workspace_id);
    if let Some(v) = f.category { url.push_str(&format!("category={}&", v)); }
    if let Some(v) = f.transaction_type { url.push_str(&format!("type={}&", v)); }
    if let Some(v) = f.from { url.push_str(&format!("from={}&", v)); }
    if let Some(v) = f.to { url.push_str(&format!("to={}&", v)); }
    if let Some(v) = f.limit { url.push_str(&format!("limit={}&", v)); }
    if let Some(v) = f.offset { url.push_str(&format!("offset={}&", v)); }
    ureq::get(&url)
        .set("Authorization", &bearer(token))
        .call()
        .map_err(err_body)?
        .into_string()
        .map_err(|e| e.to_string())
}

pub fn create(
    base: &str, token: &str, workspace_id: &str,
    amount: f64, currency: &str, category: &str,
    description: Option<&str>, date: &str, transaction_type: &str,
) -> Result<String, String> {
    let mut body = json!({
        "amount": amount,
        "currency": currency,
        "category": category,
        "transaction_date": date,
        "transaction_type": transaction_type,
    });
    if let Some(d) = description { body["description"] = json!(d); }
    ureq::post(&format!("{}/api/v1/workspaces/{}/transactions", base, workspace_id))
        .set("Authorization", &bearer(token))
        .send_json(body)
        .map_err(err_body)?
        .into_string()
        .map_err(|e| e.to_string())
}

pub fn delete(base: &str, token: &str, workspace_id: &str, id: &str) -> Result<String, String> {
    ureq::delete(&format!("{}/api/v1/workspaces/{}/transactions/{}", base, workspace_id, id))
        .set("Authorization", &bearer(token))
        .call()
        .map_err(err_body)?
        .into_string()
        .map_err(|e| e.to_string())
}
