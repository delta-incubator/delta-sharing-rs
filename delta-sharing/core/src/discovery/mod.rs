use std::sync::Arc;

use crate::error::Result;
use crate::types as t;

#[cfg(feature = "memory")]
pub use in_memory::*;

#[cfg(feature = "memory")]
pub(crate) mod in_memory;

#[async_trait::async_trait]
pub trait DiscoveryHandler: Send + Sync {
    type Recipient: Send;

    /// List all shares that the recipient is allowed to read.
    async fn list_shares(
        &self,
        request: t::ListSharesRequest,
        recipient: Self::Recipient,
    ) -> Result<t::ListSharesResponse>;

    /// Get a share by name.
    async fn get_share(&self, request: t::GetShareRequest) -> Result<t::GetShareResponse>;

    /// List all schemas in a share.
    async fn list_schemas(&self, request: t::ListSchemasRequest) -> Result<t::ListSchemasResponse>;

    /// List all tables in a schema.
    async fn list_schema_tables(
        &self,
        request: t::ListSchemaTablesRequest,
    ) -> Result<t::ListSchemaTablesResponse>;

    /// List all tables in a share.
    async fn list_share_tables(
        &self,
        request: t::ListShareTablesRequest,
    ) -> Result<t::ListShareTablesResponse>;
}

#[async_trait::async_trait]
impl<T: DiscoveryHandler + Send + Sync> DiscoveryHandler for Arc<T> {
    type Recipient = T::Recipient;

    async fn list_shares(
        &self,
        request: t::ListSharesRequest,
        recipient: Self::Recipient,
    ) -> Result<t::ListSharesResponse> {
        self.as_ref().list_shares(request, recipient).await
    }

    async fn get_share(&self, request: t::GetShareRequest) -> Result<t::GetShareResponse> {
        self.as_ref().get_share(request).await
    }

    async fn list_schemas(&self, request: t::ListSchemasRequest) -> Result<t::ListSchemasResponse> {
        self.as_ref().list_schemas(request).await
    }

    async fn list_schema_tables(
        &self,
        request: t::ListSchemaTablesRequest,
    ) -> Result<t::ListSchemaTablesResponse> {
        self.as_ref().list_schema_tables(request).await
    }

    async fn list_share_tables(
        &self,
        request: t::ListShareTablesRequest,
    ) -> Result<t::ListShareTablesResponse> {
        self.as_ref().list_share_tables(request).await
    }
}
