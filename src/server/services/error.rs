use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use utoipa::ToSchema;

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ErrorMessage {
    pub error_code: String,
    pub message: String,
}

pub enum Error {
    InternalServerProblem(anyhow::Error),
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    ValidationFailed,
    Conflict,
    EnvironmentVariableMissing,
    NotImplemented,
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_tuple("delta_sharing::Error");
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
            Error::Forbidden => {
                f.field(&"Forbidden");
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

#[allow(clippy::recursive_format_impl)]
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
                tracing::error!("stacktrace: {}", e.backtrace());
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            Error::BadRequest => (StatusCode::BAD_REQUEST, "Bad request"),
            Error::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            Error::Forbidden => (StatusCode::FORBIDDEN, "Forbidden"),
            Error::NotFound => (StatusCode::NOT_FOUND, "Not found"),
            Error::ValidationFailed => (StatusCode::BAD_REQUEST, "Bad request"),
            Error::Conflict => (StatusCode::CONFLICT, "Conflict"),
            Error::EnvironmentVariableMissing => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            Error::NotImplemented => (StatusCode::NOT_IMPLEMENTED, "Not implemented"),
        };
        (
            status,
            Json(ErrorMessage {
                error_code: status.as_str().into(),
                message: message.into(),
            }),
        )
            .into_response()
    }
}
