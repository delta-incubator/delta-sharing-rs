pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Generic error: {source}")]
    Generic {
        /// The wrapped error
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },

    /// Error when the object is not found at given location
    #[error("Object not found: {source}")]
    NotFound {
        /// The wrapped error
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },

    /// Error when the object already exists
    #[error("Object at location already exists: {source}")]
    AlreadyExists {
        /// The wrapped error
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },

    /// Error when the required conditions failed for the operation
    #[error("Request precondition failure: {source}")]
    Precondition {
        /// The wrapped error
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },

    /// Error when the object at the location isn't modified
    #[error("Object not modified: {source}")]
    NotModified {
        /// The wrapped error
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },

    /// Error when the used credentials don't have enough permission
    /// to perform the requested operation
    #[error("The operation lacked the necessary privileges to complete: {source}")]
    PermissionDenied {
        /// The wrapped error
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },

    /// Error when the used credentials lack valid authentication
    #[error("The operation lacked valid authentication credentials: {source}")]
    Unauthenticated {
        /// The wrapped error
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },

    /// Error when a configuration key is invalid for the store used
    #[error("Configuration key: '{}' is not valid.", key)]
    UnknownConfigurationKey {
        /// The configuration key used
        key: String,
    },
}
