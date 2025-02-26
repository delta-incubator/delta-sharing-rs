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
            include_browse: query as Option<bool>,
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

/// A catalog is the first layer of Unity Catalog’s three-level namespace.
/// It’s used to organize your data assets. Users can see all catalogs on which
/// they have been assigned the USE_CATALOG data permission.
#[async_trait::async_trait]
pub trait CatalogHandler: Send + Sync + 'static {
    /// List catalogs.
    ///
    /// Gets an array of catalogs in the metastore. If the caller is the metastore admin,
    /// all catalogs will be retrieved. Otherwise, only catalogs owned by the caller
    /// (or for which the caller has the USE_CATALOG privilege) will be retrieved.
    /// There is no guarantee of a specific ordering of the elements in the array.
    async fn list_catalogs(
        &self,
        request: ListCatalogsRequest,
        context: RequestContext,
    ) -> Result<ListCatalogsResponse>;

    /// Create a new catalog.
    ///
    /// Creates a new catalog instance in the parent metastore if the caller
    /// is a metastore admin or has the CREATE_CATALOG privilege.
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

    /// Update a catalog.
    async fn update_catalog(
        &self,
        request: UpdateCatalogRequest,
        context: RequestContext,
    ) -> Result<CatalogInfo>;
}
