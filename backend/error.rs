use axum::{http::StatusCode, response::IntoResponse};
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    LoginFail,

    // Auth Errors
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    AuthFailCtxNotInRequestExt,

    // We want to move these into the model layer
    TicketDeleteFailedIdNotFound { id: u64 },
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        println!("{:<12} - {self:?}", "INTO_RES");

        // Create placeholder Axum response
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Now insert our server error into the response
        response.extensions_mut().insert(self);

        // Map Response middleware is where the magic will happen
        response
    }
}

// Error Boilerplate
impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), std::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

// Note the CasingDifference in convention from ServerErrors
#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}

// These methods convert server errors into client errors
impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        match self {
            // Auth
            Self::AuthFailCtxNotInRequestExt
            | Self::AuthFailNoAuthTokenCookie
            | Self::AuthFailCtxNotInRequestExt => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

            // Model
            // Self::TicketDeleteFailedIdNotFound { id } => {
            //     (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
            // }

            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}
