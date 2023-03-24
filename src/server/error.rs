use crate::server::schemas::error::Error as ErrorResponse;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use tracing::error;

pub enum Error {
    InternalServerProblem(anyhow::Error),
    BadRequest,
    Unauthorized,
    NotFound,
    ValidationFailed,
    Conflict,
    EnvironmentVariableMissing,
    NotImplemented,
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_tuple("kotosiro_sharing::Error");
        match self {
            Error::InternalServerProblem(_) => {
                f.field(&"Something went wrong");
            }
            Error::BadRequest => {
                f.field(&"Bad request");
            }
            Error::Unauthorized => {
                f.field(&"Unauthorized");
            }
            Error::NotFound => {
                f.field(&"Not found");
            }
            Error::ValidationFailed => {
                f.field(&"Validation failed");
            }
            Error::Conflict => {
                f.field(&"Confliction occured");
            }
            Error::EnvironmentVariableMissing => {
                f.field(&"Missing environment variable");
            }
            Error::NotImplemented => {
                f.field(&"Not implemented");
            }
        };
        f.finish()
    }
}

#[allow(clippy::to_string_in_display)]
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InternalServerProblem(ref e) => {
                write!(f, "{}: {}", self, e)
            }
            _ => f.write_str(&self.to_string()),
        }
    }
}

impl From<anyhow::Error> for Error {
    fn from(e: anyhow::Error) -> Self {
        Error::InternalServerProblem(e)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Error::InternalServerProblem(e) => {
                error!("stacktrace: {}", e.backtrace());
                (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
            }
            Error::BadRequest => (StatusCode::BAD_REQUEST, "Bad request"),
            Error::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            Error::NotFound => (StatusCode::NOT_FOUND, "Not found"),
            Error::ValidationFailed => (StatusCode::UNPROCESSABLE_ENTITY, "Validation errors"),
            Error::Conflict => (StatusCode::CONFLICT, "Confliction occured"),
            Error::EnvironmentVariableMissing => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Missing environment variable",
            ),
            Error::NotImplemented => (StatusCode::NOT_IMPLEMENTED, "Not implemented"),
        };
        (
            status,
            Json(ErrorResponse {
                error_code: status.as_str().into(),
                message: message.into(),
            }),
        )
            .into_response()
    }
}
