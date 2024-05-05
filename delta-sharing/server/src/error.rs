use axum::extract::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use delta_sharing_core::{Error as CoreError, ErrorResponse};
use tracing::error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    Core(CoreError),
}

impl From<CoreError> for Error {
    fn from(error: CoreError) -> Self {
        Error::Core(error)
    }
}

const INTERNAL_ERROR: (StatusCode, &'static str) = (
    StatusCode::INTERNAL_SERVER_ERROR,
    "The request is not handled correctly due to a server error.",
);

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Error::Core(CoreError::NotFound) => (
                StatusCode::NOT_FOUND,
                "The requested resource does not exist.",
            ),
            Error::Core(CoreError::NotAllowed) => (
                StatusCode::FORBIDDEN,
                "The request is forbidden from being fulfilled.",
            ),
            Error::Core(CoreError::Unauthenticated) => (
                StatusCode::UNAUTHORIZED,
                "The request is unauthenticated. The bearer token is missing or incorrect.",
            ),
            Error::Core(CoreError::Kernel(error)) => {
                let message = format!("Kernel error: {}", error);
                error!("delta-kernel error: {}", message);
                INTERNAL_ERROR
            }
            Error::Core(CoreError::InvalidTableLocation(location)) => {
                let message = format!("Invalid table location: {}", location);
                error!("{}", message);
                INTERNAL_ERROR
            }
            Error::Core(CoreError::Generic(message)) => {
                error!("Generic error: {}", message);
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
