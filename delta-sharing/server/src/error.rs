use axum::extract::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use delta_sharing_core::Error as CoreError;
use serde::Serialize;

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub enum Error {
    Core(CoreError),
}

impl From<CoreError> for Error {
    fn from(error: CoreError) -> Self {
        Error::Core(error)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        // How we want errors responses to be serialized
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct ErrorResponse {
            error_code: String,
            message: String,
        }

        let (status, message) = match self {
            Error::Core(CoreError::NotFound) => (
                StatusCode::NOT_FOUND,
                "The requested resource does not exist.",
            ),
            Error::Core(CoreError::NotAllowed) => (
                StatusCode::FORBIDDEN,
                "The request is forbidden from being fulfilled.",
            ),
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
