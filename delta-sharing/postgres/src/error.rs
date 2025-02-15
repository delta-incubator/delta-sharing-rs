/// A convenience type for declaring Results in the Delta Sharing libraries.
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Connection(sqlx::Error),

    #[error(transparent)]
    Migration(#[from] sqlx::migrate::MigrateError),

    #[error("Entity not found: '{0}'")]
    EntityNotFound(String),

    #[error("Invalid Url: '{0}'")]
    InvalidUrl(#[from] url::ParseError),

    #[error("Failed to decode page token: '{0}'")]
    DecodePageToken(#[from] base64::DecodeError),

    #[error("Generic error: {0}")]
    Generic(String),
}

impl Error {
    pub fn entity_not_found(msg: impl Into<String>) -> Self {
        Error::EntityNotFound(msg.into())
    }

    pub fn generic(msg: impl Into<String>) -> Self {
        Error::Generic(msg.into())
    }
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => Error::EntityNotFound("Row not found".to_string()),
            _ => Error::Connection(e),
        }
    }
}
