use std::collections::HashMap;
use std::sync::Arc;

use crate::models::catalog::v1 as catalog;
use crate::models::v1::*;
use crate::{
    Decision, DiscoveryHandler, Permission, Policy, Recipient, ResourceIdent, ResourceRef, Result,
    SharingRepository, TableQueryHandler,
};

#[derive(Clone)]
pub struct DeltaSharingHandler {
    pub discovery: Arc<dyn DiscoveryHandler>,
    pub query: Arc<dyn TableQueryHandler>,
    pub policy: Arc<dyn Policy>,
}

#[async_trait::async_trait]
impl DiscoveryHandler for DeltaSharingHandler {
    async fn list_shares(&self, request: ListSharesRequest) -> Result<ListSharesResponse> {
        self.discovery.list_shares(request).await
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
        resource: &ResourceIdent,
        permission: &Permission,
        recipient: &Recipient,
    ) -> Result<Decision> {
        self.policy.authorize(resource, permission, recipient).await
    }
}

#[derive(Clone)]
pub struct DeltaRepositoryHandler {
    pub repo: Arc<dyn SharingRepository>,
    pub query: Arc<dyn TableQueryHandler>,
    pub policy: Arc<dyn Policy>,
}

#[async_trait::async_trait]
impl SharingRepository for DeltaRepositoryHandler {
    async fn add_share(
        &self,
        name: &str,
        comment: Option<String>,
        properties: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<catalog::ShareInfo> {
        self.repo.add_share(name, comment, properties).await
    }

    async fn get_share(&self, id: &ResourceRef) -> Result<catalog::ShareInfo> {
        self.repo.get_share(id).await
    }

    async fn delete_share(&self, id: &ResourceRef) -> Result<()> {
        self.repo.delete_share(id).await
    }

    async fn list_shares(
        &self,
        max_results: Option<usize>,
        page_token: Option<String>,
    ) -> Result<(Vec<Share>, Option<String>)> {
        self.repo.list_shares(max_results, page_token).await
    }

    async fn add_schema(
        &self,
        share: &ResourceRef,
        name: &str,
        comment: Option<String>,
        properties: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<catalog::SchemaInfo> {
        self.repo.add_schema(share, name, comment, properties).await
    }

    async fn get_schema(&self, id: &ResourceRef) -> Result<catalog::SchemaInfo> {
        self.repo.get_schema(id).await
    }

    async fn delete_schema(&self, id: &ResourceRef) -> Result<()> {
        self.repo.delete_schema(id).await
    }

    async fn list_schemas(
        &self,
        share: &ResourceRef,
        max_results: Option<usize>,
        page_token: Option<String>,
    ) -> Result<(Vec<Schema>, Option<String>)> {
        self.repo.list_schemas(share, max_results, page_token).await
    }

    async fn list_schema_tables(
        &self,
        schema: &ResourceRef,
        max_results: Option<usize>,
        page_token: Option<String>,
    ) -> Result<(Vec<Table>, Option<String>)> {
        self.repo
            .list_schema_tables(schema, max_results, page_token)
            .await
    }

    async fn list_share_tables(
        &self,
        share: &ResourceRef,
        max_results: Option<usize>,
        page_token: Option<String>,
    ) -> Result<(Vec<Table>, Option<String>)> {
        self.repo
            .list_share_tables(share, max_results, page_token)
            .await
    }
}

#[async_trait::async_trait]
impl TableQueryHandler for DeltaRepositoryHandler {
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
impl Policy for DeltaRepositoryHandler {
    async fn authorize(
        &self,
        resource: &ResourceIdent,
        permission: &Permission,
        recipient: &Recipient,
    ) -> Result<Decision> {
        self.policy.authorize(resource, permission, recipient).await
    }
}
