#[cfg(feature = "axum")]
use axum::extract::rejection::{PathRejection, QueryRejection};
#[cfg(feature = "grpc")]
use tonic::Status;

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

    #[error("Invalid Argument: {0}")]
    InvalidArgument(String),

    #[error("Generic error: {0}")]
    Generic(String),

    #[error("Failed to extract recipient from request")]
    MissingRecipient,

    #[error(transparent)]
    SerDe(#[from] serde_json::Error),

    #[cfg(feature = "axum")]
    #[error("Axum path: {0}")]
    AxumPath(#[from] PathRejection),

    #[cfg(feature = "axum")]
    #[error("Axum query: {0}")]
    AxumQuery(#[from] QueryRejection),
}

impl Error {
    pub fn generic(msg: impl Into<String>) -> Self {
        Self::Generic(msg.into())
    }

    pub fn invalid_argument(msg: impl Into<String>) -> Self {
        Self::InvalidArgument(msg.into())
    }
}

#[cfg(feature = "grpc")]
impl From<Error> for Status {
    fn from(e: Error) -> Self {
        match e {
            Error::NotFound => Status::not_found("The requested resource does not exist."),
            Error::NotAllowed => {
                Status::permission_denied("The request is forbidden from being fulfilled.")
            }
            Error::Unauthenticated => Status::unauthenticated(
                "The request is unauthenticated. The bearer token is missing or incorrect.",
            ),
            Error::Kernel(error) => Status::internal(error.to_string()),
            Error::SerDe(_) => Status::internal("Encountered invalid table log."),
            Error::InvalidTableLocation(location) => {
                Status::internal(format!("Invalid table location: {}", location))
            }
            Error::MissingRecipient => {
                Status::invalid_argument("Failed to extract recipient from request")
            }
            Error::InvalidArgument(message) => Status::invalid_argument(message),
            Error::Generic(message) => Status::internal(message),
            #[cfg(feature = "axum")]
            Error::AxumPath(rejection) => Status::internal(format!("Axum path: {}", rejection)),
            #[cfg(feature = "axum")]
            Error::AxumQuery(rejection) => Status::internal(format!("Axum query: {}", rejection)),
        }
    }
}

// impl From<JwtError> for Error {
//     fn from(e: JwtError) -> Self {
//         match e.kind() {
//             JwtErrorKind::InvalidToken
//             | JwtErrorKind::InvalidIssuer
//             | JwtErrorKind::InvalidSubject
//             | JwtErrorKind::ExpiredSignature
//             | JwtErrorKind::ImmatureSignature
//             | JwtErrorKind::InvalidSignature => Error::Unauthenticated,
//             _ => Error::Generic(e.to_string()),
//         }
//     }
// }

#[cfg(feature = "axum")]
mod server {
    use axum::extract::Json;
    use axum::http::StatusCode;
    use axum::response::{IntoResponse, Response};
    use tracing::error;

    use super::Error;
    use crate::models::ErrorResponse;

    const INTERNAL_ERROR: (StatusCode, &str) = (
        StatusCode::INTERNAL_SERVER_ERROR,
        "The request is not handled correctly due to a server error.",
    );

    const INVALID_ARGUMENT: (StatusCode, &str) = (
        StatusCode::BAD_REQUEST,
        "Invalid argument provided in the request.",
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
                Error::InvalidArgument(message) => {
                    error!("Invalid argument: {}", message);
                    INVALID_ARGUMENT
                }
                Error::SerDe(_) => {
                    error!("Invalid table log encountered");
                    INTERNAL_ERROR
                }
                Error::Generic(message) => {
                    error!("Generic error: {}", message);
                    INTERNAL_ERROR
                }
                Error::MissingRecipient => {
                    error!("Failed to extract recipient from request");
                    (
                        StatusCode::BAD_REQUEST,
                        "Failed to extract recipient from request",
                    )
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
