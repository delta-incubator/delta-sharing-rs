use std::sync::Arc;

mod api;
pub mod error;
#[cfg(feature = "grpc")]
mod grpc;
pub mod handler;
mod kernel;
#[cfg(feature = "memory")]
pub mod memory;
pub mod models;
pub mod policy;
mod resources;
#[cfg(feature = "axum")]
pub mod rest;
pub mod secrets;

pub use self::resources::*;
pub use api::*;
pub use delta_sharing_derive;
pub use error::*;
pub use handler::*;
pub use kernel::*;
pub use models::*;
pub use policy::*;
pub use secrets::*;

/// Resolver for the storage location of a table.
#[async_trait::async_trait]
pub trait TableLocationResolver: Send + Sync {
    async fn resolve(&self, table: &ResourceRef) -> Result<url::Url>;
}

#[async_trait::async_trait]
impl<T: TableLocationResolver> TableLocationResolver for Arc<T> {
    async fn resolve(&self, table: &ResourceRef) -> Result<url::Url> {
        T::resolve(self, table).await
    }
}
