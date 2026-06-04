use serde::Serialize;
use worker::Response;

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    Conflict(String),
    TooManyRequests(String),
    Internal,
}

#[derive(Serialize)]
struct ErrorBody {
    message: String,
    stack_exceptions: Vec<String>,
}

impl AppError {
    pub fn into_response(self) -> Response {
        let (status, kind, message) = match self {
            Self::NotFound(m) => (404u16, "NotFound", m),
            Self::BadRequest(m) => (400, "BadRequest", m),
            Self::Unauthorized(m) => (401, "Unauthorized", m),
            Self::Forbidden(m) => (403, "Forbidden", m),
            Self::Conflict(m) => (409, "Conflict", m),
            Self::TooManyRequests(m) => (429, "TooManyRequests", m),
            Self::Internal => (500, "Internal", "internal server error".to_owned()),
        };
        let body = ErrorBody {
            message: message.clone(),
            stack_exceptions: vec![kind.to_owned(), message],
        };
        Response::from_json(&body).map_or_else(
            |_| Response::error("internal server error", 500).expect("infallible"),
            |r| r.with_status(status),
        )
    }
}

impl From<worker::Error> for AppError {
    fn from(_: worker::Error) -> Self {
        Self::Internal
    }
}

impl From<serde_json::Error> for AppError {
    fn from(_: serde_json::Error) -> Self {
        Self::BadRequest("invalid request data".into())
    }
}
