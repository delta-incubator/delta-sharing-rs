/// A convenience type for declaring Results in the Delta Sharing libraries.
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Connection(#[from] tonic::transport::Error),

    #[error(transparent)]
    OpenFGA(#[from] tonic::Status),

    #[error("Store '{0}' not found in service.")]
    StoreNotFound(String),
}

impl Error {
    pub fn store_not_found(msg: impl Into<String>) -> Self {
        Self::StoreNotFound(msg.into())
    }
}
