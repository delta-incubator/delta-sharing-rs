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
