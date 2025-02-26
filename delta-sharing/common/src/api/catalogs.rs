use delta_sharing_derive::rest_handlers;

use super::{RequestContext, SecuredAction};
use crate::models::catalogs::v1::*;
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
}
