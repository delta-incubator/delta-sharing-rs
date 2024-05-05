// A convenience type for declaring Results in the Delta Sharing libraries.
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("kernel error: {0}")]
    Kernel(#[from] delta_kernel::Error),

    #[error("Entity not found.")]
    NotFound,

    #[error("Recipient is not allowed to read the entity.")]
    NotAllowed,

    #[error("Invalid table location: {0}")]
    InvalidTableLocation(String),
}
