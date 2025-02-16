use std::sync::Arc;

use delta_sharing_common::models::v1::*;
use delta_sharing_common::{
    Decision, DiscoveryHandler, Permission, Policy, Recipient, Resource, Result, TableQueryHandler,
};

#[derive(Clone)]
pub struct DeltaSharingHandler {
    pub discovery: Arc<dyn DiscoveryHandler>,
    pub query: Arc<dyn TableQueryHandler>,
    pub policy: Arc<dyn Policy>,
}

#[async_trait::async_trait]
impl DiscoveryHandler for DeltaSharingHandler {
    async fn list_shares(
        &self,
        request: ListSharesRequest,
        recipient: &Recipient,
    ) -> Result<ListSharesResponse> {
        self.discovery.list_shares(request, recipient).await
    }

    async fn get_share(&self, request: GetShareRequest) -> Result<Share> {
        self.discovery.get_share(request).await
    }

    async fn list_schemas(&self, request: ListSchemasRequest) -> Result<ListSchemasResponse> {
        self.discovery.list_schemas(request).await
    }

    async fn list_schema_tables(
        &self,
        request: ListSchemaTablesRequest,
    ) -> Result<ListSchemaTablesResponse> {
        self.discovery.list_schema_tables(request).await
    }

    async fn list_share_tables(
        &self,
        request: ListShareTablesRequest,
    ) -> Result<ListShareTablesResponse> {
        self.discovery.list_share_tables(request).await
    }
}

#[async_trait::async_trait]
impl TableQueryHandler for DeltaSharingHandler {
    async fn get_table_version(
        &self,
        request: GetTableVersionRequest,
    ) -> Result<GetTableVersionResponse> {
        self.query.get_table_version(request).await
    }

    async fn get_table_metadata(&self, request: GetTableMetadataRequest) -> Result<QueryResponse> {
        self.query.get_table_metadata(request).await
    }
}

#[async_trait::async_trait]
impl Policy for DeltaSharingHandler {
    async fn authorize(
        &self,
        resource: &Resource,
        permission: &Permission,
        recipient: &Recipient,
    ) -> Result<Decision> {
        self.policy.authorize(resource, permission, recipient).await
    }
}
