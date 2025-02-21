use std::sync::Arc;

use bytes::Bytes;
use uuid::Uuid;

mod api;
pub mod error;
#[cfg(feature = "grpc")]
mod grpc;
pub mod handlers;
#[cfg(feature = "memory")]
mod in_memory;
mod kernel;
pub mod managers;
pub mod models;
pub mod policy;
pub mod repository;
mod resources;
#[cfg(feature = "axum")]
pub mod rest;

pub use self::resources::*;
pub use api::*;
pub use delta_sharing_derive;
pub use error::*;
pub use handlers::*;
#[cfg(feature = "memory")]
pub use in_memory::*;
pub use kernel::*;
pub use managers::*;
pub use models::catalog::v1 as catalog;
pub use models::Resource;
pub use models::*;
pub use models::{
    IntoJSONStruct, IntoJson, IntoProto, IntoProtoStruct, JsonValue, PropertyMapHandler,
};
pub use policy::*;
pub use repository::*;

#[derive(Clone, Debug)]
pub enum Recipient {
    Anonymous,
    User(String),
    Custom(Bytes),
}

impl Recipient {
    pub fn anonymous() -> Self {
        Self::Anonymous
    }

    pub fn user(name: impl Into<String>) -> Self {
        Self::User(name.into())
    }

    pub fn custom(data: Bytes) -> Self {
        Self::Custom(data)
    }
}

/// Unique identifier for a resource.
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum ResourceRef {
    Uuid(Uuid),
    Name(ResourceName),
    /// Not referencing a specific resource.
    ///
    /// This is used to represent a wildcard in a policy
    /// which can be useful to check if a user can create
    /// or manage resources at a specific level.
    Undefined,
}

impl std::fmt::Display for ResourceRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Uuid(u) => write!(f, "{}", u.hyphenated()),
            Self::Name(name) => {
                write!(f, "{}", name)
            }
            Self::Undefined => write!(f, "*"),
        }
    }
}

impl From<Uuid> for ResourceRef {
    fn from(val: Uuid) -> Self {
        Self::Uuid(val)
    }
}

impl From<&Uuid> for ResourceRef {
    fn from(val: &Uuid) -> Self {
        Self::Uuid(*val)
    }
}

impl From<ResourceName> for ResourceRef {
    fn from(val: ResourceName) -> Self {
        Self::Name(val)
    }
}

// --8<-- [start:discovery-handler]
/// Handler for discovering shares, schemas, and tables exposed by a Delta Sharing server.
#[async_trait::async_trait]
pub trait DiscoveryHandler: Send + Sync + 'static {
    /// List all shares that the recipient is allowed to read.
    async fn list_shares(&self, request: ListSharesRequest) -> Result<ListSharesResponse>;

    /// Get a share by name.
    async fn get_share(&self, request: GetShareRequest) -> Result<Share>;

    /// List all schemas in a share.
    async fn list_schemas(
        &self,
        request: ListSharingSchemasRequest,
    ) -> Result<ListSharingSchemasResponse>;

    /// List all tables in a schema.
    async fn list_schema_tables(
        &self,
        request: ListSchemaTablesRequest,
    ) -> Result<ListSchemaTablesResponse>;

    /// List all tables in a share.
    async fn list_share_tables(
        &self,
        request: ListShareTablesRequest,
    ) -> Result<ListShareTablesResponse>;
}
// --8<-- [end:discovery-handler]

#[async_trait::async_trait]
impl<T: DiscoveryHandler> DiscoveryHandler for Arc<T> {
    async fn list_shares(&self, request: ListSharesRequest) -> Result<ListSharesResponse> {
        T::list_shares(self, request).await
    }

    async fn get_share(&self, request: GetShareRequest) -> Result<Share> {
        T::get_share(self, request).await
    }

    async fn list_schemas(
        &self,
        request: ListSharingSchemasRequest,
    ) -> Result<ListSharingSchemasResponse> {
        T::list_schemas(self, request).await
    }

    async fn list_schema_tables(
        &self,
        request: ListSchemaTablesRequest,
    ) -> Result<ListSchemaTablesResponse> {
        T::list_schema_tables(self, request).await
    }

    async fn list_share_tables(
        &self,
        request: ListShareTablesRequest,
    ) -> Result<ListShareTablesResponse> {
        T::list_share_tables(self, request).await
    }
}

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

// --8<-- [start:sharing-repository-handler]
/// Manage resources that the Delta Sharing server exposes.
///
/// The delta-sharing protocol does not specofy endpoints for creating or deleting resources.
/// Unless one is willing to accept a static setup, it is necessary to have a way to manage
/// resources dynamically. This trait provides a way to manage resources dynamically.
#[async_trait::async_trait]
pub trait RepositoryHandler: Send + Sync + 'static {
    /// Create a share.
    async fn create_share(
        &self,
        request: CreateShareRequest,
    ) -> Result<crate::models::sharing::v1::ShareInfo>;

    /// Delete a share.
    async fn delete_share(&self, request: DeleteShareRequest) -> Result<()>;

    /// Create a schema.
    async fn create_schema(
        &self,
        request: CreateSharingSchemaRequest,
    ) -> Result<crate::models::sharing::v1::SharingSchemaInfo>;

    /// Delete a schema.
    async fn delete_schema(&self, request: DeleteSharingSchemaRequest) -> Result<()>;
}
// --8<-- [end:sharing-repository-handler]
