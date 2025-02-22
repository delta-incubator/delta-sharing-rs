use crate::models::sharing::v1::*;
use crate::Result;

use super::RequestContext;

#[async_trait::async_trait]
pub trait SharingDiscoveryHandler: Send + Sync + 'static {
    /// List all shares that the recipient is allowed to read.
    async fn list_shares(
        &self,
        request: ListSharesRequest,
        context: RequestContext,
    ) -> Result<ListSharesResponse>;

    /// Get a share by name.
    async fn get_share(&self, request: GetShareRequest, context: RequestContext) -> Result<Share>;

    /// List all schemas in a share.
    async fn list_sharing_schemas(
        &self,
        request: ListSharingSchemasRequest,
        context: RequestContext,
    ) -> Result<ListSharingSchemasResponse>;

    /// List all tables in a schema.
    async fn list_schema_tables(
        &self,
        request: ListSchemaTablesRequest,
        context: RequestContext,
    ) -> Result<ListSchemaTablesResponse>;

    /// List all tables in a share.
    async fn list_share_tables(
        &self,
        request: ListShareTablesRequest,
        context: RequestContext,
    ) -> Result<ListShareTablesResponse>;
}

#[async_trait::async_trait]
pub trait SharingExtensionHandler: Send + Sync + 'static {
    /// Create a share.
    async fn create_share(
        &self,
        request: CreateShareRequest,
        context: RequestContext,
    ) -> Result<ShareInfo>;

    /// Delete a share.
    async fn delete_share(
        &self,
        request: DeleteShareRequest,
        context: RequestContext,
    ) -> Result<()>;

    /// Create a schema.
    async fn create_sharing_schema(
        &self,
        request: CreateSharingSchemaRequest,
        context: RequestContext,
    ) -> Result<SharingSchemaInfo>;

    /// Delete a schema.
    async fn delete_sharing_schema(
        &self,
        request: DeleteSharingSchemaRequest,
        context: RequestContext,
    ) -> Result<()>;
}

#[async_trait::async_trait]
pub trait SharingQueryHandler: Send + Sync + 'static {
    async fn get_table_version(
        &self,
        request: GetTableVersionRequest,
        context: RequestContext,
    ) -> Result<GetTableVersionResponse>;

    async fn get_table_metadata(
        &self,
        request: GetTableMetadataRequest,
        context: RequestContext,
    ) -> Result<QueryResponse>;
}
