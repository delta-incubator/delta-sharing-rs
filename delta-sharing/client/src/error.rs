// A convenience type for declaring Results in the Delta Sharing libraries.
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Generic error: {}", source)]
    Generic {
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },

    #[error("kernel error: {0}")]
    Kernel(#[from] delta_kernel::Error),

    #[error("Object at location {} not found: {}", path, source)]
    NotFound {
        path: String,
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },

    #[error("Object at location {} already exists: {}", path, source)]
    AlreadyExists {
        path: String,
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },

    #[error("Request precondition failure for path {}: {}", path, source)]
    Precondition {
        path: String,
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },

    #[error("No or invalid token provided.")]
    Unauthenticated,

    #[error("Recipient is not allowed to read the entity.")]
    NotAllowed,

    #[error("Invalid table location: {0}")]
    InvalidTableLocation(String),

    #[error("Configuration key: '{}' is not valid.", key)]
    UnknownConfigurationKey { key: String },
}
