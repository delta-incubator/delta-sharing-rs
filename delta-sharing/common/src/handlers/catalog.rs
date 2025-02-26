use itertools::Itertools;

use crate::api::catalogs::CatalogHandler;
use crate::models::catalogs::v1::*;
use crate::{ObjectLabel, Policy, RequestContext, ResourceStore, Result, SecuredAction};

#[async_trait::async_trait]
impl<T: ResourceStore + Policy> CatalogHandler for T {
    async fn create_catalog(
        &self,
        request: CreateCatalogRequest,
        context: RequestContext,
    ) -> Result<CatalogInfo> {
        self.check_required(&request, context.as_ref()).await?;
        let resource = CatalogInfo {
            name: request.name,
            comment: request.comment,
            properties: request.properties,
            ..Default::default()
        };
        self.create(resource.into()).await?.0.try_into()
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
        let (resources, next_page_token) = self
            .list(
                &ObjectLabel::CatalogInfo,
                None,
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
        self.check_required(&request, context.as_ref()).await?;
        let ident = request.resource();
        let resource = CatalogInfo {
            name: request.new_name,
            comment: request.comment,
            properties: request.properties,
            ..Default::default()
        };
        self.update(&ident, resource.into()).await?.0.try_into()
    }
}
