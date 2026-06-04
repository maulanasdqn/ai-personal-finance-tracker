use crate::error::AppError;
use worker::Request;

const MAX_JSON_BODY: usize = 64 * 1024;

pub fn require_json(req: &Request) -> Result<(), AppError> {
    let ct = req
        .headers()
        .get("content-type")
        .ok()
        .flatten()
        .unwrap_or_default()
        .to_lowercase();
    if !ct.contains("application/json") {
        return Err(AppError::BadRequest(
            "Content-Type must be application/json".into(),
        ));
    }
    Ok(())
}

pub fn limit_body(req: &Request) -> Result<(), AppError> {
    if let Ok(Some(cl)) = req.headers().get("content-length") {
        if let Ok(size) = cl.parse::<usize>() {
            if size > MAX_JSON_BODY {
                return Err(AppError::BadRequest(format!(
                    "request body too large (max {MAX_JSON_BODY} bytes)"
                )));
            }
        }
    }
    Ok(())
}
