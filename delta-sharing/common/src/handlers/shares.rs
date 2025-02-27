use itertools::Itertools;

use crate::api::shares::SharesHandler;
use crate::models::shares::v1::*;
use crate::policy::{process_resources, Permission, Policy};
use crate::{ObjectLabel, RequestContext, ResourceStore, Result, SecuredAction};

#[async_trait::async_trait]
impl<T: ResourceStore + Policy> SharesHandler for T {
    async fn create_share(
        &self,
        request: CreateShareRequest,
        context: RequestContext,
    ) -> Result<ShareInfo> {
        self.check_required(&request, context.as_ref()).await?;
        let resource = ShareInfo {
            name: request.name,
            comment: request.comment,
            ..Default::default()
        };
        // TODO:
        // - update the share with the current actor as owner
        // - create updated_* relations
        self.create(resource.into()).await?.0.try_into()
    }

    async fn delete_share(
        &self,
        request: DeleteShareRequest,
        context: RequestContext,
    ) -> Result<()> {
        self.check_required(&request, context.as_ref()).await?;
        self.delete(&request.resource()).await
    }

    async fn list_shares(
        &self,
        request: ListSharesRequest,
        context: RequestContext,
    ) -> Result<ListSharesResponse> {
        self.check_required(&request, context.as_ref()).await?;
        let (mut resources, next_page_token) = self
            .list(
                &ObjectLabel::ShareInfo,
                None,
                request.max_results.map(|v| v as usize),
                request.page_token,
            )
            .await?;
        process_resources(self, context.as_ref(), &Permission::Read, &mut resources).await?;
        Ok(ListSharesResponse {
            shares: resources.into_iter().map(|r| r.try_into()).try_collect()?,
            next_page_token,
        })
    }

    async fn get_share(
        &self,
        request: GetShareRequest,
        context: RequestContext,
    ) -> Result<ShareInfo> {
        self.check_required(&request, context.as_ref()).await?;
        self.get(&request.resource()).await?.0.try_into()
    }

    async fn update_share(
        &self,
        request: UpdateShareRequest,
        context: RequestContext,
    ) -> Result<ShareInfo> {
        self.check_required(&request, context.as_ref()).await?;
        let ident = request.resource();
        let resource = ShareInfo {
            name: request.new_name.unwrap_or_else(|| request.name.clone()),
            comment: request.comment,
            owner: request.owner,
            ..Default::default()
        };
        // TODO:
        // - handle data object updates
        // - add update_* relations
        self.update(&ident, resource.into()).await?.0.try_into()
    }
}
