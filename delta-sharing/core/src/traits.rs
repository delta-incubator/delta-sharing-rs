use crate::error::Result;
use crate::types as t;

#[async_trait::async_trait]
pub trait DiscoveryHandler {
    type Recipient;

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
