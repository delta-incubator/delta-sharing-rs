use std::sync::Arc;

use crate::models::catalog::v1::*;
use crate::policy::Policy;
use crate::resources::ResourceStore;
use crate::Result;

mod catalog;
mod credentials;
mod sharing;

pub use catalog::*;
pub use credentials::*;
pub use sharing::*;

pub struct RequestContext {
    pub actor: Option<String>,
}

pub struct ServerHandler {
    pub policy: Arc<dyn Policy>,
    pub store: Arc<dyn ResourceStore>,
}

impl ServerHandler {
    pub fn new(policy: Arc<dyn Policy>, store: Arc<dyn ResourceStore>) -> Self {
        Self { policy, store }
    }
}

#[async_trait::async_trait]
impl CatalogHandler for ServerHandler {
    async fn create_catalog(
        &self,
        request: CreateCatalogRequest,
        context: RequestContext,
    ) -> Result<CatalogInfo> {
        let resource = CatalogInfo {
            name: request.name,
            comment: request.comment,
            properties: request.properties,
            ..Default::default()
        };
        let (resource, _) = self.store.create(resource.into()).await?;
        resource.try_into()
    }

    async fn delete_catalog(
        &self,
        request: DeleteCatalogRequest,
        context: RequestContext,
    ) -> Result<()> {
        self.delete_catalog(request, context).await
    }

    async fn get_catalog(
        &self,
        request: GetCatalogRequest,
        context: RequestContext,
    ) -> Result<CatalogInfo> {
        self.get_catalog(request, context).await
    }

    async fn list_catalogs(
        &self,
        request: ListCatalogsRequest,
        context: RequestContext,
    ) -> Result<ListCatalogsResponse> {
        self.list_catalogs(request, context).await
    }

    async fn update_catalog(
        &self,
        request: UpdateCatalogRequest,
        context: RequestContext,
    ) -> Result<CatalogInfo> {
        self.update_catalog(request, context).await
    }

    async fn create_schema(
        &self,
        request: CreateSchemaRequest,
        context: RequestContext,
    ) -> Result<SchemaInfo> {
        self.create_schema(request, context).await
    }

    async fn delete_schema(
        &self,
        request: DeleteSchemaRequest,
        context: RequestContext,
    ) -> Result<()> {
        self.delete_schema(request, context).await
    }

    async fn list_schemas(
        &self,
        request: ListSchemasRequest,
        context: RequestContext,
    ) -> Result<ListSchemasResponse> {
        self.list_schemas(request, context).await
    }

    async fn get_schema(
        &self,
        request: GetSchemaRequest,
        context: RequestContext,
    ) -> Result<SchemaInfo> {
        self.get_schema(request, context).await
    }

    async fn update_schema(
        &self,
        request: UpdateSchemaRequest,
        context: RequestContext,
    ) -> Result<SchemaInfo> {
        self.update_schema(request, context).await
    }
}
