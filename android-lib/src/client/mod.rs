pub mod auth;
pub mod insight;
pub mod statement;
pub mod transaction;
pub mod workspace;

pub(crate) fn bearer(token: &str) -> String {
    format!("Bearer {}", token)
}

pub(crate) fn err_body(e: ureq::Error) -> String {
    match e {
        // HTTP error body is already the API's standardized error JSON.
        ureq::Error::Status(_, r) => r.into_string().unwrap_or_else(|_| {
            r#"{"message":"request failed","stack_exceptions":["Transport","request failed"]}"#
                .to_owned()
        }),
        // Shape transport/network errors into the same envelope.
        ureq::Error::Transport(t) => {
            let msg = t.to_string().replace('"', "\\\"");
            format!(
                r#"{{"message":"{msg}","stack_exceptions":["Transport","{msg}"]}}"#
            )
        }
    }
}
