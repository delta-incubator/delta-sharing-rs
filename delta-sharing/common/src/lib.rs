// `async_trait` adds `#[must_use]` to every trait method, which clippy reports as a duplicate of
// the `must_use` on the boxed future it returns. The attribute comes from the macro, so it is
// allowed for the crate instead of on each trait.
#![allow(clippy::double_must_use)]

use std::sync::Arc;

pub mod api;
pub mod error;
#[cfg(feature = "grpc")]
mod grpc;
pub mod handler;
mod kernel;
#[cfg(feature = "memory")]
pub mod memory;
pub mod models;
pub mod policy;
pub mod resources;
#[cfg(feature = "axum")]
pub mod rest;
pub mod secrets;

pub use self::resources::*;
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
