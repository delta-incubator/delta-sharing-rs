// A convenience type for declaring Results in the Delta Sharing libraries.
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Entity not found.")]
    NotFound,

    #[error("Recipient is not allowed to read the entity.")]
    NotAllowed,

    #[cfg(feature = "memory")]
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[cfg(feature = "memory")]
    #[error("Invalid config file: {0}")]
    SerdeYaml(#[from] serde_yml::Error),
}
