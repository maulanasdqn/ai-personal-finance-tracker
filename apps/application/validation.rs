use crate::error::AppError;

const MAX_FILE_SIZE: usize = 10 * 1024 * 1024;

static ALLOWED_CONTENT_TYPES: &[&str] = &[
    "image/jpeg",
    "image/png",
    "image/webp",
];

pub fn validate_password_strength(password: &str) -> Result<(), AppError> {
    if !password.chars().any(|c| c.is_ascii_uppercase()) {
        return Err(AppError::BadRequest("password must contain at least one uppercase letter".into()));
    }
    if !password.chars().any(|c| c.is_ascii_lowercase()) {
        return Err(AppError::BadRequest("password must contain at least one lowercase letter".into()));
    }
    if !password.chars().any(|c| c.is_ascii_digit()) {
        return Err(AppError::BadRequest("password must contain at least one digit".into()));
    }
    Ok(())
}

pub fn validate_file_upload(bytes: &[u8], content_type: &str) -> Result<(), AppError> {
    if bytes.is_empty() {
        return Err(AppError::BadRequest("file body is empty".into()));
    }
    if bytes.len() > MAX_FILE_SIZE {
        let size = bytes.len();
        return Err(AppError::BadRequest(format!(
            "file size exceeds the 10 MB limit (received {size} bytes)"
        )));
    }
    let base_type = content_type.split(';').next().unwrap_or("").trim();
    if !ALLOWED_CONTENT_TYPES.contains(&base_type) {
        let allowed = ALLOWED_CONTENT_TYPES.join(", ");
        return Err(AppError::BadRequest(format!(
            "unsupported file type '{base_type}'. Allowed: {allowed}"
        )));
    }
    Ok(())
}

pub fn sanitize_file_name(name: &str) -> String {
    let name = name.trim();
    let sanitized: String = name
        .chars()
        .map(|c| if c.is_alphanumeric() || matches!(c, '.' | '-' | '_') { c } else { '_' })
        .collect();
    if sanitized.is_empty() { "upload".to_string() } else { sanitized[..sanitized.len().min(200)].to_string() }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_node);
    use super::*;

    #[wasm_bindgen_test]
    fn password_accepts_valid() {
        assert!(validate_password_strength("Password1").is_ok());
    }

    #[wasm_bindgen_test]
    fn password_rejects_no_uppercase() {
        assert!(validate_password_strength("password1").is_err());
    }

    #[wasm_bindgen_test]
    fn password_rejects_no_lowercase() {
        assert!(validate_password_strength("PASSWORD1").is_err());
    }

    #[wasm_bindgen_test]
    fn password_rejects_no_digit() {
        assert!(validate_password_strength("Password").is_err());
    }

    #[wasm_bindgen_test]
    fn file_upload_accepts_jpeg() {
        assert!(validate_file_upload(b"data", "image/jpeg").is_ok());
    }

    #[wasm_bindgen_test]
    fn file_upload_accepts_png() {
        assert!(validate_file_upload(b"data", "image/png").is_ok());
    }

    #[wasm_bindgen_test]
    fn file_upload_accepts_webp() {
        assert!(validate_file_upload(b"data", "image/webp").is_ok());
    }

    #[wasm_bindgen_test]
    fn file_upload_rejects_pdf() {
        assert!(validate_file_upload(b"data", "application/pdf").is_err());
    }

    #[wasm_bindgen_test]
    fn file_upload_rejects_empty_bytes() {
        assert!(validate_file_upload(b"", "image/jpeg").is_err());
    }

    #[wasm_bindgen_test]
    fn file_upload_rejects_over_limit() {
        let big = vec![0u8; 10 * 1024 * 1024 + 1];
        assert!(validate_file_upload(&big, "image/jpeg").is_err());
    }

    #[wasm_bindgen_test]
    fn file_upload_rejects_unknown_type() {
        assert!(validate_file_upload(b"data", "application/octet-stream").is_err());
    }

    #[wasm_bindgen_test]
    fn sanitize_replaces_special_chars() {
        assert_eq!(sanitize_file_name("hello world!"), "hello_world_");
    }

    #[wasm_bindgen_test]
    fn sanitize_truncates_to_200() {
        let long = "a".repeat(300);
        assert_eq!(sanitize_file_name(&long).len(), 200);
    }

    #[wasm_bindgen_test]
    fn sanitize_returns_upload_for_empty() {
        assert_eq!(sanitize_file_name(""), "upload");
    }

    #[wasm_bindgen_test]
    fn sanitize_preserves_allowed_chars() {
        assert_eq!(sanitize_file_name("file-name_v2.txt"), "file-name_v2.txt");
    }
}
