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
        ureq::Error::Status(_, r) => r.into_string().unwrap_or_else(|_| "request failed".into()),
        ureq::Error::Transport(t) => t.to_string(),
    }
}
