use std::sync::Arc;

use bytes::Bytes;

pub mod error;
#[cfg(feature = "grpc")]
mod grpc;
#[cfg(feature = "memory")]
mod in_memory;
mod kernel;
pub mod models;
pub mod policy;
#[cfg(feature = "axum")]
pub mod rest;

pub use error::*;
#[cfg(feature = "memory")]
pub use in_memory::*;
pub use kernel::*;
pub use models::v1::*;
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
#[derive(Debug, Clone, PartialEq)]
pub enum ResourceIdent {
    Uunid(uuid::Uuid),
    Name(Vec<String>, String),
}

impl std::fmt::Display for ResourceIdent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Uunid(u) => write!(f, "{}", u),
            Self::Name(path, name) => {
                if path.is_empty() {
                    write!(f, "{}", name)
                } else {
                    write!(f, "{}.{}", path.join("."), name)
                }
            }
        }
    }
}

impl From<uuid::Uuid> for ResourceIdent {
    fn from(val: uuid::Uuid) -> Self {
        Self::Uunid(val)
    }
}

impl From<String> for ResourceIdent {
    fn from(val: String) -> Self {
        Self::Name(vec![], val)
    }
}

impl From<&String> for ResourceIdent {
    fn from(val: &String) -> Self {
        Self::Name(vec![], val.clone())
    }
}

impl From<&str> for ResourceIdent {
    fn from(val: &str) -> Self {
        Self::Name(vec![], val.to_string())
    }
}

impl<T: ToString + Sized, U: ToString, const N: usize> From<([T; N], U)> for ResourceIdent {
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
    async fn list_shares(
        &self,
        request: ListSharesRequest,
        recipient: &Recipient,
    ) -> Result<ListSharesResponse>;

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
    async fn list_shares(
        &self,
        request: ListSharesRequest,
        recipient: &Recipient,
    ) -> Result<ListSharesResponse> {
        T::list_shares(self, request, recipient).await
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
    async fn resolve(&self, table: &models::TableRef) -> Result<url::Url>;
}

#[async_trait::async_trait]
impl<T: TableLocationResover> TableLocationResover for Arc<T> {
    async fn resolve(&self, table: &models::TableRef) -> Result<url::Url> {
        T::resolve(self, table).await
    }
}

/// Handler for querying tables exposed by a Delta Sharing server.
#[async_trait::async_trait]
pub trait TableQueryHandler: Send + Sync {
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
