use delta_sharing_derive::rest_handlers;
use itertools::Itertools;

use super::{RequestContext, SecuredAction};
use crate::models::catalogs::v1::*;
use crate::policy::{process_resources, Permission, Policy};
use crate::resources::{ResourceIdent, ResourceName, ResourceRef};
use crate::{Error, ObjectLabel, Recipient, ResourceStore, Result};

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

    /// Get a catalog
    ///
    /// Gets the specified catalog in a metastore. The caller must be a metastore admin,
    /// the owner of the catalog, or a user that has the USE_CATALOG privilege set for their account.
    async fn get_catalog(
        &self,
        request: GetCatalogRequest,
        context: RequestContext,
    ) -> Result<CatalogInfo>;

    /// Delete a catalog
    ///
    /// Deletes the catalog that matches the supplied name. The caller must be a metastore admin
    /// or the owner of the catalog.
    async fn delete_catalog(
        &self,
        request: DeleteCatalogRequest,
        context: RequestContext,
    ) -> Result<()>;

    /// Update a catalog
    ///
    /// Updates the catalog that matches the supplied name. The caller must be either
    /// the owner of the catalog, or a metastore admin (when changing the owner field of the catalog).
    async fn update_catalog(
        &self,
        request: UpdateCatalogRequest,
        context: RequestContext,
    ) -> Result<CatalogInfo>;
}

#[async_trait::async_trait]
impl<T: ResourceStore + Policy> CatalogHandler for T {
    async fn create_catalog(
        &self,
        request: CreateCatalogRequest,
        context: RequestContext,
    ) -> Result<CatalogInfo> {
        self.check_required(&request, context.as_ref()).await?;
        let catalog_type = if request.provider_name.is_some() {
            CatalogType::DeltasharingCatalog
        } else {
            CatalogType::ManagedCatalog
        };
        let resource = CatalogInfo {
            name: request.name,
            comment: request.comment,
            properties: request.properties,
            storage_root: request.storage_root,
            provider_name: request.provider_name,
            share_name: request.share_name,
            catalog_type: Some(catalog_type as i32),
            ..Default::default()
        };
        let info = self.create(resource.into()).await?.0.try_into()?;

        // TODO:
        // - make current actor the owner of the catalog including permissions
        // - create updated_* relations

        Ok(info)
    }

    async fn delete_catalog(
        &self,
        request: DeleteCatalogRequest,
        context: RequestContext,
    ) -> Result<()> {
        self.check_required(&request, context.as_ref()).await?;
        self.delete(&request.resource()).await
    }

    async fn get_catalog(
        &self,
        request: GetCatalogRequest,
        context: RequestContext,
    ) -> Result<CatalogInfo> {
        self.check_required(&request, context.recipient()).await?;
        self.get(&request.resource()).await?.0.try_into()
    }

    async fn list_catalogs(
        &self,
        request: ListCatalogsRequest,
        context: RequestContext,
    ) -> Result<ListCatalogsResponse> {
        self.check_required(&request, context.as_ref()).await?;
        let (mut resources, next_page_token) = self
            .list(
                &ObjectLabel::CatalogInfo,
                None,
                request.max_results.map(|v| v as usize),
                request.page_token,
            )
            .await?;
        process_resources(self, context.as_ref(), &Permission::Read, &mut resources).await?;
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
        self.check_required(&request, context.as_ref()).await?;
        let ident = request.resource();
        let resource = CatalogInfo {
            name: request.new_name,
            comment: request.comment,
            properties: request.properties,
            ..Default::default()
        };
        // TODO:
        // - add update_* relations
        // - update owner if necessary
        self.update(&ident, resource.into()).await?.0.try_into()
    }
}
