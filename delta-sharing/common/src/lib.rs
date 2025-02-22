use std::sync::Arc;

use bytes::Bytes;
use uuid::Uuid;

mod api;
pub mod error;
#[cfg(feature = "grpc")]
mod grpc;
pub mod handlers;
mod kernel;
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

impl ResourceRef {
    pub fn is_undefined(&self) -> bool {
        matches!(self, Self::Undefined)
    }

    pub fn name(name: impl Into<ResourceName>) -> Self {
        Self::Name(name.into())
    }
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
