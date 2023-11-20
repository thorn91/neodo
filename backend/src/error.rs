use std::fmt;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    InvalidSetup(String),

    NotFoundError(String),
    ValidationError(String),

    InternalError,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::InvalidSetup(msg) => write!(f, "InvalidSetup: {}", msg),
            AppError::NotFoundError(msg) => write!(f, "NotFoundError: {}", msg),
            AppError::ValidationError(msg) => write!(f, "ValidationError: {}", msg),
            AppError::InternalError => write!(f, "InternalError"),
        }
    }
}

impl std::error::Error for AppError {}

impl From<axum::http::uri::InvalidUri> for AppError {
    fn from(err: axum::http::uri::InvalidUri) -> AppError {
        AppError::InvalidSetup(err.to_string())
    }
}