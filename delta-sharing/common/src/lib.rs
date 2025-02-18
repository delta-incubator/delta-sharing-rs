use std::sync::Arc;

use bytes::Bytes;

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

pub use error::*;
pub use handlers::*;
#[cfg(feature = "memory")]
pub use in_memory::*;
pub use kernel::*;
pub use managers::*;
pub use models::catalog::v1 as catalog;
pub use models::catalog::v1::resource::Resource;
pub use models::v1::*;
pub use models::{
    IntoJSONStruct, IntoJson, IntoProto, IntoProtoStruct, JsonValue, PropertyMapHandler,
};
pub use policy::*;
pub use repository::*;
pub use resources::*;

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
    Uuid(uuid::Uuid),
    Name(Vec<String>, String),
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
            Self::Name(path, name) => {
                if path.is_empty() {
                    write!(f, "{}", name)
                } else {
                    write!(f, "{}.{}", path.join("."), name)
                }
            }
            Self::Undefined => write!(f, "*"),
        }
    }
}

impl From<uuid::Uuid> for ResourceRef {
    fn from(val: uuid::Uuid) -> Self {
        Self::Uuid(val)
    }
}

impl From<String> for ResourceRef {
    fn from(val: String) -> Self {
        Self::Name(vec![], val)
    }
}

impl From<&String> for ResourceRef {
    fn from(val: &String) -> Self {
        Self::Name(vec![], val.clone())
    }
}

impl From<&str> for ResourceRef {
    fn from(val: &str) -> Self {
        Self::Name(vec![], val.to_string())
    }
}

impl<T: ToString + Sized, U: ToString, const N: usize> From<([T; N], U)> for ResourceRef {
    fn from(val: ([T; N], U)) -> Self {
        Self::Name(
            val.0.iter().map(|s| s.to_string()).collect(),
            val.1.to_string(),
        )
    }
}

/// Handler for discovering shares, schemas, and tables exposed by a Delta Sharing server.
#[async_trait::async_trait]
pub trait DiscoveryHandler: Send + Sync + 'static {
    /// List all shares that the recipient is allowed to read.
    async fn list_shares(&self, request: ListSharesRequest) -> Result<ListSharesResponse>;

    /// Get a share by name.
    async fn get_share(&self, request: GetShareRequest) -> Result<Share>;

    /// List all schemas in a share.
    async fn list_schemas(&self, request: ListSchemasRequest) -> Result<ListSchemasResponse>;

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

#[async_trait::async_trait]
impl<T: DiscoveryHandler> DiscoveryHandler for Arc<T> {
    async fn list_shares(&self, request: ListSharesRequest) -> Result<ListSharesResponse> {
        T::list_shares(self, request).await
    }

    async fn get_share(&self, request: GetShareRequest) -> Result<Share> {
        T::get_share(self, request).await
    }

    async fn list_schemas(&self, request: ListSchemasRequest) -> Result<ListSchemasResponse> {
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

/// Handler for querying tables exposed by a Delta Sharing server.
#[async_trait::async_trait]
pub trait TableQueryHandler: Send + Sync + 'static {
    async fn get_table_version(
        &self,
        request: GetTableVersionRequest,
    ) -> Result<GetTableVersionResponse>;

    async fn get_table_metadata(&self, request: GetTableMetadataRequest) -> Result<QueryResponse>;
}

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

#[async_trait::async_trait]
pub trait RepositoryHandler: Send + Sync + 'static {
    async fn create_share(
        &self,
        request: catalog::CreateShareRequest,
    ) -> Result<catalog::ShareInfo>;
    async fn delete_share(&self, request: catalog::DeleteShareRequest) -> Result<()>;
    async fn create_schema(
        &self,
        request: catalog::CreateSchemaRequest,
    ) -> Result<catalog::SchemaInfo>;
    async fn delete_schema(&self, request: catalog::DeleteSchemaRequest) -> Result<()>;
}

#[async_trait::async_trait]
pub trait CredentialsHandler: Send + Sync + 'static {
    async fn create_credentials(
        &self,
        request: catalog::CreateCredentialRequest,
    ) -> Result<catalog::Credential>;

    async fn delete_credentials(
        &self,
        request: catalog::DeleteCredentialRequest,
    ) -> Result<catalog::Credential>;

    async fn get_credentials(
        &self,
        request: catalog::GetCredentialRequest,
    ) -> Result<catalog::Credential>;

    async fn create_storage_location(
        &self,
        request: catalog::CreateStorageLocationRequest,
    ) -> Result<catalog::StorageLocation>;

    async fn delete_storage_location(
        &self,
        request: catalog::DeleteStorageLocationRequest,
    ) -> Result<catalog::StorageLocation>;

    async fn get_storage_location(
        &self,
        request: catalog::GetStorageLocationRequest,
    ) -> Result<catalog::StorageLocation>;

    async fn list_storage_locations(
        &self,
        request: catalog::ListStorageLocationsRequest,
    ) -> Result<catalog::ListStorageLocationsResponse>;
}

#[async_trait::async_trait]
pub trait StorageLocationHandler: Send + Sync + 'static {
    async fn create_storage_location(
        &self,
        request: catalog::CreateStorageLocationRequest,
    ) -> Result<catalog::StorageLocation>;

    async fn delete_storage_location(
        &self,
        request: catalog::DeleteStorageLocationRequest,
    ) -> Result<catalog::StorageLocation>;

    async fn get_storage_location(
        &self,
        request: catalog::GetStorageLocationRequest,
    ) -> Result<catalog::StorageLocation>;

    async fn list_storage_locations(
        &self,
        request: catalog::ListStorageLocationsRequest,
    ) -> Result<catalog::ListStorageLocationsResponse>;
}
