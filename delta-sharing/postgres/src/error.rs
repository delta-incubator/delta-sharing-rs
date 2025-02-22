/// A convenience type for declaring Results in the Delta Sharing libraries.
use delta_sharing_common::Error as CommonError;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Connection(sqlx::Error),

    #[error(transparent)]
    Migration(#[from] sqlx::migrate::MigrateError),

    #[error("Invalid Url: '{0}'")]
    InvalidUrl(#[from] url::ParseError),

    #[error("Failed to decode page token: '{0}'")]
    DecodePageToken(#[from] base64::DecodeError),

    #[error("Generic error: {0}")]
    Generic(String),

    #[error("Entity not found: '{0}'")]
    EntityNotFound(String),

    #[error("Already exists: '{0}'")]
    AlreadyExists(String),
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
        match &e {
            sqlx::Error::RowNotFound => Error::EntityNotFound("Row not found".to_string()),
            // https://www.postgresql.org/docs/current/errcodes-appendix.html
            sqlx::Error::Database(db_err) => {
                let pg_err = db_err.try_downcast_ref::<sqlx::postgres::PgDatabaseError>();
                match pg_err {
                    Some(pg_err) if pg_err.code() == "23505" => {
                        Error::AlreadyExists("Unique violation".to_string())
                    }
                    Some(pg_err) if pg_err.code() == "23503" => {
                        Error::EntityNotFound("Foreign key violation".to_string())
                    }
                    _ => Error::Connection(e),
                }
            }
            _ => Error::Connection(e),
        }
    }
}

impl From<Error> for CommonError {
    fn from(e: Error) -> Self {
        match e {
            Error::Connection(e) => CommonError::generic(e.to_string()),
            Error::Migration(e) => CommonError::generic(e.to_string()),
            Error::InvalidUrl(e) => CommonError::InvalidArgument(e.to_string()),
            Error::DecodePageToken(e) => CommonError::InvalidArgument(e.to_string()),
            Error::Generic(e) => CommonError::Generic(e),
            Error::EntityNotFound(_) => CommonError::NotFound,
            Error::AlreadyExists(_) => CommonError::NotAllowed,
        }
    }
}
