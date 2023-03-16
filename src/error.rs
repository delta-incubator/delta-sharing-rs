use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use serde_json::json;
use tracing::debug;

pub enum Error {
    InternalServerProblem(anyhow::Error),
    BadRequest,
    Unauthorized,
    ValidationFailed,
    Conflict,
    EnvironmentVariableMissing,
    NotImplemented,
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
                debug!("stacktrace: {}", e.backtrace());
                (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
            }
            Error::BadRequest => (StatusCode::BAD_REQUEST, "Bad request"),
            Error::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            Error::ValidationFailed => (StatusCode::UNPROCESSABLE_ENTITY, "Validation errors"),
            Error::Conflict => (StatusCode::CONFLICT, "Confliction occured"),
            Error::EnvironmentVariableMissing => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Missing environment variable",
            ),
            Error::NotImplemented => (StatusCode::NOT_IMPLEMENTED, "Not implemented"),
        };
        let body = Json(json!({
            "error": message,
        }));
        (status, body).into_response()
    }
}
