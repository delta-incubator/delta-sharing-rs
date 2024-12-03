/// A convenience type for declaring Results in the Delta Sharing libraries.
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Connection(#[from] sqlx::Error),

    #[error(transparent)]
    Migration(#[from] sqlx::migrate::MigrateError),

    #[error("Entity not found: '{0}'")]
    EntityNotFound(String),
}

impl Error {
    pub fn entity_not_found(msg: impl Into<String>) -> Self {
        Error::EntityNotFound(msg.into())
    }
}
