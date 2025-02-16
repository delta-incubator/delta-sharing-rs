use crate::models::v1::*;
use crate::policy::{process_resources, Permission, Policy};
use crate::{DiscoveryHandler, Recipient, Result, TableQueryHandler};

#[async_trait::async_trait]
pub trait DiscoveryManager: Send + Sync + 'static {
    async fn list_shares(
        &self,
        request: ListSharesRequest,
        recipient: &Recipient,
    ) -> Result<ListSharesResponse>;

    /// Get a share by name.
    async fn get_share(&self, request: GetShareRequest, recipient: &Recipient) -> Result<Share>;

    /// List all schemas in a share.
    async fn list_schemas(
        &self,
        request: ListSchemasRequest,
        recipient: &Recipient,
    ) -> Result<ListSchemasResponse>;

    /// List all tables in a schema.
    async fn list_schema_tables(
        &self,
        request: ListSchemaTablesRequest,
        recipient: &Recipient,
    ) -> Result<ListSchemaTablesResponse>;

    /// List all tables in a share.
    async fn list_share_tables(
        &self,
        request: ListShareTablesRequest,
        recipient: &Recipient,
    ) -> Result<ListShareTablesResponse>;
}

#[async_trait::async_trait]
impl<T: DiscoveryHandler + Policy> DiscoveryManager for T {
    async fn list_shares(
        &self,
        request: ListSharesRequest,
        recipient: &Recipient,
    ) -> Result<ListSharesResponse> {
        let mut shares = self.list_shares(request).await?;
        process_resources(self, recipient, &Permission::Read, &mut shares.items).await?;
        Ok(shares)
    }

    async fn get_share(&self, request: GetShareRequest, recipient: &Recipient) -> Result<Share> {
        self.check_required(&request, recipient).await?;
        Ok(self.get_share(request).await?)
    }

    async fn list_schemas(
        &self,
        request: ListSchemasRequest,
        recipient: &Recipient,
    ) -> Result<ListSchemasResponse> {
        self.check_required(&request, recipient).await?;
        Ok(self.list_schemas(request).await?)
    }

    async fn list_schema_tables(
        &self,
        request: ListSchemaTablesRequest,
        recipient: &Recipient,
    ) -> Result<ListSchemaTablesResponse> {
        self.check_required(&request, recipient).await?;
        Ok(self.list_schema_tables(request).await?)
    }

    async fn list_share_tables(
        &self,
        request: ListShareTablesRequest,
        recipient: &Recipient,
    ) -> Result<ListShareTablesResponse> {
        self.check_required(&request, recipient).await?;
        Ok(self.list_share_tables(request).await?)
    }
}

#[async_trait::async_trait]
pub trait TableQueryManager: Send + Sync + 'static {
    async fn get_table_version(
        &self,
        request: GetTableVersionRequest,
        recipient: &Recipient,
    ) -> Result<GetTableVersionResponse>;

    async fn get_table_metadata(
        &self,
        request: GetTableMetadataRequest,
        recipient: &Recipient,
    ) -> Result<QueryResponse>;
}

#[async_trait::async_trait]
impl<T: TableQueryHandler + Policy> TableQueryManager for T {
    async fn get_table_version(
        &self,
        request: GetTableVersionRequest,
        recipient: &Recipient,
    ) -> Result<GetTableVersionResponse> {
        self.check_required(&request, recipient).await?;
        Ok(self.get_table_version(request).await?)
    }

    async fn get_table_metadata(
        &self,
        request: GetTableMetadataRequest,
        recipient: &Recipient,
    ) -> Result<QueryResponse> {
        self.check_required(&request, recipient).await?;
        Ok(self.get_table_metadata(request).await?)
    }
}
