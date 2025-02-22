use itertools::Itertools;

use super::ServerHandler;
use crate::api::{CatalogHandler, RequestContext};
use crate::models::catalog::v1::*;
use crate::{ResourceIdent, ResourceName, ResourceRef, Result, SecuredAction};

#[async_trait::async_trait]
impl CatalogHandler for ServerHandler {
    async fn create_catalog(
        &self,
        request: CreateCatalogRequest,
        context: RequestContext,
    ) -> Result<CatalogInfo> {
        self.policy
            .check_required(&request, context.as_ref())
            .await?;
        let resource = CatalogInfo {
            name: request.name,
            comment: request.comment,
            properties: request.properties,
            ..Default::default()
        };
        self.store.create(resource.into()).await?.0.try_into()
    }

    async fn delete_catalog(
        &self,
        request: DeleteCatalogRequest,
        context: RequestContext,
    ) -> Result<()> {
        self.policy
            .check_required(&request, context.as_ref())
            .await?;
        self.store.delete(&request.resource()).await
    }

    async fn get_catalog(
        &self,
        request: GetCatalogRequest,
        context: RequestContext,
    ) -> Result<CatalogInfo> {
        self.policy
            .check_required(&request, context.recipient())
            .await?;
        self.store.get(&request.resource()).await?.0.try_into()
    }

    async fn list_catalogs(
        &self,
        request: ListCatalogsRequest,
        context: RequestContext,
    ) -> Result<ListCatalogsResponse> {
        self.policy
            .check_required(&request, context.as_ref())
            .await?;
        let (resources, next_page_token) = self
            .store
            .list(
                &ResourceIdent::catalog(ResourceRef::Undefined),
                request.max_results.map(|v| v as usize),
                request.page_token,
            )
            .await?;
        Ok(ListCatalogsResponse {
            catalogs: resources.into_iter().map(|r| r.try_into()).try_collect()?,
            next_page_token,
        })
    }

    async fn update_catalog(
        &self,
        request: UpdateCatalogRequest,
        context: RequestContext,
    ) -> Result<CatalogInfo> {
        self.policy
            .check_required(&request, context.as_ref())
            .await?;
        todo!()
    }

    async fn create_schema(
        &self,
        request: CreateSchemaRequest,
        context: RequestContext,
    ) -> Result<SchemaInfo> {
        self.policy
            .check_required(&request, context.as_ref())
            .await?;
        let resource = SchemaInfo {
            name: request.name,
            catalog_name: request.catalog_name,
            comment: request.comment,
            properties: request.properties,
            ..Default::default()
        };
        self.store.create(resource.into()).await?.0.try_into()
    }

    async fn delete_schema(
        &self,
        request: DeleteSchemaRequest,
        context: RequestContext,
    ) -> Result<()> {
        self.policy
            .check_required(&request, context.as_ref())
            .await?;
        self.store.delete(&request.resource()).await
    }

    async fn list_schemas(
        &self,
        request: ListSchemasRequest,
        context: RequestContext,
    ) -> Result<ListSchemasResponse> {
        self.policy
            .check_required(&request, context.as_ref())
            .await?;
        let (resources, next_page_token) = self
            .store
            .list(
                &ResourceIdent::catalog(ResourceName::new([&request.catalog_name])),
                request.max_results.map(|v| v as usize),
                request.page_token,
            )
            .await?;
        Ok(ListSchemasResponse {
            schemas: resources.into_iter().map(|r| r.try_into()).try_collect()?,
            next_page_token,
        })
    }

    async fn get_schema(
        &self,
        request: GetSchemaRequest,
        context: RequestContext,
    ) -> Result<SchemaInfo> {
        self.policy
            .check_required(&request, context.as_ref())
            .await?;
        self.store.get(&request.resource()).await?.0.try_into()
    }

    async fn update_schema(
        &self,
        request: UpdateSchemaRequest,
        context: RequestContext,
    ) -> Result<SchemaInfo> {
        self.policy
            .check_required(&request, context.as_ref())
            .await?;
        todo!()
    }
}
