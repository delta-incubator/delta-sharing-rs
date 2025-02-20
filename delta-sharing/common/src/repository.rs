use std::collections::HashMap;

use crate::models::sharing::v1::*;
use crate::{DiscoveryHandler, RepositoryHandler, ResourceRef, Result};

#[async_trait::async_trait]
pub trait SharingRepository: Send + Sync + 'static {
    async fn add_share(
        &self,
        name: &str,
        comment: Option<String>,
        properties: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<ShareInfo>;

    async fn get_share(&self, id: &ResourceRef) -> Result<ShareInfo>;
    async fn delete_share(&self, id: &ResourceRef) -> Result<()>;

    /// List shares.
    async fn list_shares(
        &self,
        max_results: Option<usize>,
        page_token: Option<String>,
    ) -> Result<(Vec<Share>, Option<String>)>;

    /// Add a schema.
    async fn add_schema(
        &self,
        share: &ResourceRef,
        name: &str,
        comment: Option<String>,
        properties: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<SharingSchemaInfo>;

    /// Get a schema.
    async fn get_schema(&self, id: &ResourceRef) -> Result<SharingSchemaInfo>;

    /// Delete a schema.
    async fn delete_schema(&self, id: &ResourceRef) -> Result<()>;

    /// List schemas.
    async fn list_schemas(
        &self,
        share: &ResourceRef,
        max_results: Option<usize>,
        page_token: Option<String>,
    ) -> Result<(Vec<SharingSchema>, Option<String>)>;

    async fn list_schema_tables(
        &self,
        schema: &ResourceRef,
        max_results: Option<usize>,
        page_token: Option<String>,
    ) -> Result<(Vec<SharingTable>, Option<String>)>;

    async fn list_share_tables(
        &self,
        share: &ResourceRef,
        max_results: Option<usize>,
        page_token: Option<String>,
    ) -> Result<(Vec<SharingTable>, Option<String>)>;

    // async fn add_table(&self, name: &str, location: &str) -> Result<TableRecord>;
    // async fn get_table(&self, id: &uuid::Uuid) -> Result<TableRecord>;
    // async fn update_table(&self, record: &TableRecord) -> Result<TableRecord>;
}

#[async_trait::async_trait]
impl<T: SharingRepository> DiscoveryHandler for T {
    async fn list_shares(&self, request: ListSharesRequest) -> Result<ListSharesResponse> {
        let (items, next_page_token) = T::list_shares(
            self,
            request.max_results.as_ref().map(|v| *v as usize),
            request.page_token,
        )
        .await?;
        Ok(ListSharesResponse {
            items,
            next_page_token,
        })
    }

    async fn get_share(&self, request: GetShareRequest) -> Result<Share> {
        let info = T::get_share(self, &request.into()).await?;
        Ok(Share {
            id: Some(info.id),
            name: info.name,
        })
    }

    async fn list_schemas(
        &self,
        request: ListSharingSchemasRequest,
    ) -> Result<ListSharingSchemasResponse> {
        let max_results = request.max_results.as_ref().map(|v| *v as usize);
        let page_token = request.page_token.clone();
        let (items, next_page_token) =
            T::list_schemas(self, &request.into(), max_results, page_token).await?;
        Ok(ListSharingSchemasResponse {
            items,
            next_page_token,
        })
    }

    async fn list_schema_tables(
        &self,
        request: ListSchemaTablesRequest,
    ) -> Result<ListSchemaTablesResponse> {
        let max_results = request.max_results.as_ref().map(|v| *v as usize);
        let page_token = request.page_token.clone();
        let (items, next_page_token) =
            T::list_schema_tables(self, &request.into(), max_results, page_token).await?;
        Ok(ListSchemaTablesResponse {
            items,
            next_page_token,
        })
    }

    async fn list_share_tables(
        &self,
        request: ListShareTablesRequest,
    ) -> Result<ListShareTablesResponse> {
        let max_results = request.max_results.as_ref().map(|v| *v as usize);
        let page_token = request.page_token.clone();
        let (items, next_page_token) =
            T::list_share_tables(self, &request.into(), max_results, page_token).await?;
        Ok(ListShareTablesResponse {
            items,
            next_page_token,
        })
    }
}

#[async_trait::async_trait]
impl<T: SharingRepository> RepositoryHandler for T {
    async fn create_share(&self, request: CreateShareRequest) -> Result<ShareInfo> {
        let share = request
            .share
            .ok_or_else(|| crate::Error::invalid_argument("share is required".to_string()))?;
        let properties = share.properties.map(|p| {
            p.fields
                .iter()
                .map(|(field, value)| (field.clone(), serde_json::to_value(value).unwrap()))
                .collect()
        });
        T::add_share(self, &share.name, share.description, properties).await
    }

    async fn delete_share(&self, request: DeleteShareRequest) -> Result<()> {
        T::delete_share(self, &request.into()).await
    }

    async fn create_schema(
        &self,
        request: CreateSharingSchemaRequest,
    ) -> Result<SharingSchemaInfo> {
        let share = ResourceRef::from(&request.share);
        let schema = request
            .schema
            .ok_or_else(|| crate::Error::invalid_argument("schema is required".to_string()))?;
        let properties = schema.properties.map(|p| {
            p.fields
                .iter()
                .map(|(field, value)| (field.clone(), serde_json::to_value(value).unwrap()))
                .collect()
        });
        T::add_schema(self, &share, &schema.name, schema.description, properties).await
    }

    async fn delete_schema(&self, request: DeleteSharingSchemaRequest) -> Result<()> {
        T::delete_schema(self, &request.into()).await
    }
}
