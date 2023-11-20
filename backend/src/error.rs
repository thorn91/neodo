use std::fmt;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidSetup(String),

    NotFoundError(String),
    ValidationError(String),

    InternalError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidSetup(msg) => write!(f, "InvalidSetup: {}", msg),
            Error::NotFoundError(msg) => write!(f, "NotFoundError: {}", msg),
            Error::ValidationError(msg) => write!(f, "ValidationError: {}", msg),
            Error::InternalError => write!(f, "InternalError"),
        }
    }
}

impl std::error::Error for Error {}

impl From<axum::http::uri::InvalidUri> for Error {
    fn from(err: axum::http::uri::InvalidUri) -> Error {
        Error::InvalidSetup(err.to_string())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Error::InvalidSetup(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            Error::NotFoundError(msg) => (StatusCode::NOT_FOUND, msg),
            Error::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            Error::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_owned(),
            ),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

impl From<sea_orm::DbErr> for Error {
    fn from(err: sea_orm::DbErr) -> Error {
        Error::InvalidSetup(err.to_string())
    }
}