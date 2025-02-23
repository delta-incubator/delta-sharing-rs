use delta_sharing_derive::rest_handlers;

use super::{RequestContext, SecuredAction};
use crate::models::catalog::v1::*;
use crate::{Error, Permission, Recipient, ResourceIdent, ResourceName, ResourceRef, Result};

rest_handlers!(
    CatalogHandler, [
        CreateCatalogRequest, Catalog, Create, CatalogInfo;
        ListCatalogsRequest, Catalog, Read, ListCatalogsResponse;
        GetCatalogRequest, Catalog, Read, CatalogInfo with [
            name: path as String,
        ];
        UpdateCatalogRequest, Catalog, Manage, CatalogInfo with [
            name: path as String,
        ];
        DeleteCatalogRequest, Catalog, Manage with [
            name: path as String,
            force: query as Option<bool>,
        ];
        CreateSchemaRequest, Schema, Create, SchemaInfo;
        ListSchemasRequest, Catalog, Read, ListSchemasResponse with [
            catalog_name: query as String
        ];
        GetSchemaRequest, Schema, Read, SchemaInfo with [
            full_name: path as String,
        ];
        UpdateSchemaRequest, Schema, Manage, SchemaInfo with [
            full_name: path as String,
        ];
        DeleteSchemaRequest, Schema, Manage with [
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
