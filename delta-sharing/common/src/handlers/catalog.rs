use itertools::Itertools;

use crate::api::catalogs::CatalogHandler;
use crate::models::catalogs::v1::*;
use crate::policy::{process_resources, Permission};
use crate::{ObjectLabel, Policy, RequestContext, ResourceStore, Result, SecuredAction};

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
