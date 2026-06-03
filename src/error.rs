use serde::Serialize;
use worker::Response;

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    Conflict(String),
    Internal(String),
}

#[derive(Serialize)]
struct ErrorBody {
    message: String,
}

impl AppError {
    pub fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::NotFound(m) => (404, m),
            Self::BadRequest(m) => (400, m),
            Self::Unauthorized(m) => (401, m),
            Self::Forbidden(m) => (403, m),
            Self::Conflict(m) => (409, m),
            Self::Internal(m) => (500, m),
        };
        Response::from_json(&ErrorBody { message })
            .map(|r| r.with_status(status))
            .unwrap_or_else(|_| Response::error("internal server error", 500).unwrap())
    }
}

impl From<worker::Error> for AppError {
    fn from(e: worker::Error) -> Self {
        Self::Internal(e.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        Self::BadRequest(e.to_string())
    }
}
