/// A convenience type for declaring Results in the Delta Sharing libraries.
pub type Result<T, E = Error> = std::result::Result<T, E>;

// `Error` wraps tonic's transport and status types, which are large. Boxing them would change
// the public error type of this crate, so the lint is allowed here instead.
#[allow(clippy::result_large_err)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Connection(#[from] tonic::transport::Error),

    #[error(transparent)]
    OpenFGA(#[from] tonic::Status),

    #[error("Store '{0}' not found in service.")]
    StoreNotFound(String),

    #[error("No authorization moidel found in store: '{0}'")]
    AuthorizationModelMissing(String),

    #[error("Either store id or name must be configured.")]
    MissingStoreConfig,
}

impl Error {
    pub fn store_not_found(msg: impl Into<String>) -> Self {
        Self::StoreNotFound(msg.into())
    }

    pub fn authorization_model_missing(msg: impl Into<String>) -> Self {
        Self::AuthorizationModelMissing(msg.into())
    }
}
