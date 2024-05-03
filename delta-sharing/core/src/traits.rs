use std::sync::Arc;

use crate::error::Result;
use crate::types as t;

#[async_trait::async_trait]
pub trait DiscoveryHandler {
    type Recipient: Send;

    /// List all shares that the recipient is allowed to read.
    async fn list_shares(
        &self,
        request: t::ListSharesRequest,
        recipient: Self::Recipient,
    ) -> Result<t::ListSharesResponse>;

    /// Get a share by name.
    async fn get_share(
        &self,
        request: t::GetShareRequest,
        recipient: Self::Recipient,
    ) -> Result<t::GetShareResponse>;

    /// List all schemas in a share.
    async fn list_schemas(
        &self,
        request: t::ListSchemasRequest,
        recipient: Self::Recipient,
    ) -> Result<t::ListSchemasResponse>;

    /// List all tables in a schema.
    async fn list_schema_tables(
        &self,
        request: t::ListSchemaTablesRequest,
        recipient: Self::Recipient,
    ) -> Result<t::ListSchemaTablesResponse>;

    /// List all tables in a share.
    async fn list_share_tables(
        &self,
        request: t::ListShareTablesRequest,
        recipient: Self::Recipient,
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

    async fn get_share(
        &self,
        request: t::GetShareRequest,
        recipient: Self::Recipient,
    ) -> Result<t::GetShareResponse> {
        self.as_ref().get_share(request, recipient).await
    }

    async fn list_schemas(
        &self,
        request: t::ListSchemasRequest,
        recipient: Self::Recipient,
    ) -> Result<t::ListSchemasResponse> {
        self.as_ref().list_schemas(request, recipient).await
    }

    async fn list_schema_tables(
        &self,
        request: t::ListSchemaTablesRequest,
        recipient: Self::Recipient,
    ) -> Result<t::ListSchemaTablesResponse> {
        self.as_ref().list_schema_tables(request, recipient).await
    }

    async fn list_share_tables(
        &self,
        request: t::ListShareTablesRequest,
        recipient: Self::Recipient,
    ) -> Result<t::ListShareTablesResponse> {
        self.as_ref().list_share_tables(request, recipient).await
    }
}
