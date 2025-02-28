use delta_sharing_common::models::sharing::v1::*;

use crate::Result;

#[async_trait::async_trait]
pub trait ServiceClient: Send + Sync + 'static {
    async fn list_shares(&self, request: ListSharesRequest) -> Result<ListSharesResponse>;
    async fn get_share(&self, request: GetShareRequest) -> Result<Share>;
    async fn list_schemas(
        &self,
        request: ListSharingSchemasRequest,
    ) -> Result<ListSharingSchemasResponse>;
    async fn list_schema_tables(
        &self,
        request: ListSchemaTablesRequest,
    ) -> Result<ListSchemaTablesResponse>;
    async fn list_share_tables(
        &self,
        request: ListShareTablesRequest,
    ) -> Result<ListShareTablesResponse>;
}
