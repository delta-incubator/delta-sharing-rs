#[cfg(feature = "axum")]
use axum::extract::rejection::{PathRejection, QueryRejection};
use jsonwebtoken::errors::{Error as JwtError, ErrorKind as JwtErrorKind};

// A convenience type for declaring Results in the Delta Sharing libraries.
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("kernel error: {0}")]
    Kernel(#[from] delta_kernel::Error),

    #[error("Entity not found.")]
    NotFound,

    #[error("No or invalid token provided.")]
    Unauthenticated,

    #[error("Recipient is not allowed to read the entity.")]
    NotAllowed,

    #[error("Invalid table location: {0}")]
    InvalidTableLocation(String),

    #[error("Generic error: {0}")]
    Generic(String),

    #[cfg(feature = "axum")]
    #[error("Axum path: {0}")]
    AxumPath(#[from] PathRejection),

    #[cfg(feature = "axum")]
    #[error("Axum query: {0}")]
    AxumQuery(#[from] QueryRejection),
}

impl From<JwtError> for Error {
    fn from(e: JwtError) -> Self {
        match e.kind() {
            JwtErrorKind::InvalidToken
            | JwtErrorKind::InvalidIssuer
            | JwtErrorKind::InvalidSubject
            | JwtErrorKind::ExpiredSignature
            | JwtErrorKind::ImmatureSignature
            | JwtErrorKind::InvalidSignature => Error::Unauthenticated,
            _ => Error::Generic(e.to_string()),
        }
    }
}

#[cfg(feature = "axum")]
mod server {
    use axum::extract::Json;
    use axum::http::StatusCode;
    use axum::response::{IntoResponse, Response};
    use tracing::error;

    use super::Error;
    use crate::types::ErrorResponse;

    const INTERNAL_ERROR: (StatusCode, &'static str) = (
        StatusCode::INTERNAL_SERVER_ERROR,
        "The request is not handled correctly due to a server error.",
    );

    impl IntoResponse for Error {
        fn into_response(self) -> Response {
            let (status, message) = match self {
                Error::NotFound => (
                    StatusCode::NOT_FOUND,
                    "The requested resource does not exist.",
                ),
                Error::NotAllowed => (
                    StatusCode::FORBIDDEN,
                    "The request is forbidden from being fulfilled.",
                ),
                Error::Unauthenticated => (
                    StatusCode::UNAUTHORIZED,
                    "The request is unauthenticated. The bearer token is missing or incorrect.",
                ),
                Error::Kernel(error) => {
                    let message = format!("Kernel error: {}", error);
                    error!("delta-kernel error: {}", message);
                    INTERNAL_ERROR
                }
                Error::InvalidTableLocation(location) => {
                    let message = format!("Invalid table location: {}", location);
                    error!("{}", message);
                    INTERNAL_ERROR
                }
                Error::Generic(message) => {
                    error!("Generic error: {}", message);
                    INTERNAL_ERROR
                }
                // TODO(roeap): what codes should these have?
                #[cfg(feature = "axum")]
                Error::AxumPath(rejection) => {
                    let message = format!("Axum path: {}", rejection);
                    error!("{}", message);
                    INTERNAL_ERROR
                }
                #[cfg(feature = "axum")]
                Error::AxumQuery(rejection) => {
                    let message = format!("Axum query: {}", rejection);
                    error!("{}", message);
                    INTERNAL_ERROR
                }
            };

            (
                status,
                Json(ErrorResponse {
                    error_code: status.to_string(),
                    message: message.to_string(),
                }),
            )
                .into_response()
        }
    }
}
