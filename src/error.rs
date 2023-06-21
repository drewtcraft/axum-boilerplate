use axum::{http::StatusCode, response::IntoResponse};
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, Serialize)]
pub enum Error {
    LogInFailure,
    DatabaseFailure,
    UserUidExpired,
    TemplateRenderingFailure,
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        response.extensions_mut().insert(self);
        response
    }
}

#[derive(Debug, Clone, strum_macros::AsRefStr)]
pub enum ClientError {
    LogInFailure,
    AuthenticationError,
    InvalidRequestParameters,
    InvalidRequestBody,
    InternalServiceError,
}

impl Error {
    pub fn status_and_client_error(&self) -> (StatusCode, ClientError) {
        match self {
            Self::LogInFailure => (StatusCode::UNAUTHORIZED, ClientError::LogInFailure),
            Self::DatabaseFailure => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::InternalServiceError,
            ),
            _ => (StatusCode::NOT_FOUND, ClientError::InternalServiceError),
        }
    }
}
