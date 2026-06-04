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
}

impl AppError {
    pub fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::NotFound(m) => (404, m),
            Self::BadRequest(m) => (400, m),
            Self::Unauthorized(m) => (401, m),
            Self::Forbidden(m) => (403, m),
            Self::Conflict(m) => (409, m),
            Self::TooManyRequests(m) => (429, m),
            Self::Internal => (500, "internal server error".to_owned()),
        };
        Response::from_json(&ErrorBody { message }).map_or_else(
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
