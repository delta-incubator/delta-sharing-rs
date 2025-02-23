use delta_sharing_derive::rest_handlers;

pub use super::RequestContext;
use crate::models::catalog::v1::*;
use crate::{Error, Recipient, Result};

rest_handlers!(
    CatalogHandler, [
        CreateCatalogRequest, CatalogInfo;
        ListCatalogsRequest, ListCatalogsResponse;
        GetCatalogRequest, CatalogInfo with [
            name: path as String,
        ];
        UpdateCatalogRequest, CatalogInfo;
        DeleteCatalogRequest with [
            name: path as String,
            force: query as Option<bool>,
        ];
        CreateSchemaRequest, SchemaInfo;
        ListSchemasRequest, ListSchemasResponse with [
            catalog_name: path as String
        ];
        GetSchemaRequest, SchemaInfo with [
            full_name: path as String,
        ];
        UpdateSchemaRequest, SchemaInfo;
        DeleteSchemaRequest with [
            full_name: path as String,
            force: query as Option<bool>,
        ];
    ]
);

#[async_trait::async_trait]
pub trait CatalogHandler: Send + Sync + 'static {
    /// Create a new catalog.
    async fn create_catalog(
        &self,
        request: CreateCatalogRequest,
        context: RequestContext,
    ) -> Result<CatalogInfo>;

    /// Delete a catalog.
    async fn delete_catalog(
        &self,
        request: DeleteCatalogRequest,
        context: RequestContext,
    ) -> Result<()>;

    /// Get a catalog.
    async fn get_catalog(
        &self,
        request: GetCatalogRequest,
        context: RequestContext,
    ) -> Result<CatalogInfo>;

    /// List catalogs.
    async fn list_catalogs(
        &self,
        request: ListCatalogsRequest,
        context: RequestContext,
    ) -> Result<ListCatalogsResponse>;

    /// Update a catalog.
    async fn update_catalog(
        &self,
        request: UpdateCatalogRequest,
        context: RequestContext,
    ) -> Result<CatalogInfo>;

    /// Create a new schema.
    async fn create_schema(
        &self,
        request: CreateSchemaRequest,
        context: RequestContext,
    ) -> Result<SchemaInfo>;

    /// Delete a schema.
    async fn delete_schema(
        &self,
        request: DeleteSchemaRequest,
        context: RequestContext,
    ) -> Result<()>;

    /// Get a schema.
    async fn get_schema(
        &self,
        request: GetSchemaRequest,
        context: RequestContext,
    ) -> Result<SchemaInfo>;

    /// List schemas.
    async fn list_schemas(
        &self,
        request: ListSchemasRequest,
        context: RequestContext,
    ) -> Result<ListSchemasResponse>;

    /// Update a schema.
    async fn update_schema(
        &self,
        request: UpdateSchemaRequest,
        context: RequestContext,
    ) -> Result<SchemaInfo>;
}
