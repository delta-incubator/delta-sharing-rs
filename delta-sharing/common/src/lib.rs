use std::sync::Arc;

mod api;
pub mod error;
#[cfg(feature = "grpc")]
mod grpc;
pub mod handlers;
mod kernel;
#[cfg(feature = "memory")]
pub mod memory;
pub mod models;
pub mod policy;
mod resources;
#[cfg(feature = "axum")]
pub mod rest;

pub use self::resources::*;
pub use api::*;
pub use delta_sharing_derive;
pub use error::*;
pub use handlers::*;
pub use kernel::*;
pub use models::*;
pub use models::{
    IntoJSONStruct, IntoJson, IntoProto, IntoProtoStruct, JsonValue, PropertyMapHandler,
};
pub use policy::*;

/// Resolver for the storage location of a table.
#[async_trait::async_trait]
pub trait TableLocationResover: Send + Sync {
    async fn resolve(&self, table: &ResourceRef) -> Result<url::Url>;
}

#[async_trait::async_trait]
impl<T: TableLocationResover> TableLocationResover for Arc<T> {
    async fn resolve(&self, table: &ResourceRef) -> Result<url::Url> {
        T::resolve(self, table).await
    }
}

// --8<-- [start:table-query-handler]
/// Handler for querying tables exposed by a Delta Sharing server.
#[async_trait::async_trait]
pub trait TableQueryHandler: Send + Sync + 'static {
    async fn get_table_version(
        &self,
        request: GetTableVersionRequest,
    ) -> Result<GetTableVersionResponse>;

    async fn get_table_metadata(&self, request: GetTableMetadataRequest) -> Result<QueryResponse>;
}
// --8<-- [end:table-query-handler]

#[async_trait::async_trait]
impl<T: TableQueryHandler> TableQueryHandler for Arc<T> {
    async fn get_table_version(
        &self,
        request: GetTableVersionRequest,
    ) -> Result<GetTableVersionResponse> {
        T::get_table_version(self, request).await
    }

    async fn get_table_metadata(&self, request: GetTableMetadataRequest) -> Result<QueryResponse> {
        T::get_table_metadata(self, request).await
    }
}
