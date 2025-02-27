use itertools::Itertools;

use crate::api::recipients::RecipientsHandler;
use crate::models::recipients::v1::*;
use crate::policy::{process_resources, Permission};
use crate::{ObjectLabel, Policy, RequestContext, ResourceStore, Result, SecuredAction};

#[async_trait::async_trait]
impl<T: ResourceStore + Policy> RecipientsHandler for T {
    async fn create_recipient(
        &self,
        request: CreateRecipientRequest,
        context: RequestContext,
    ) -> Result<RecipientInfo> {
        self.check_required(&request, context.as_ref()).await?;
        let resource = RecipientInfo {
            name: request.name,
            authentication_type: request.authentication_type,
            comment: request.comment,
            properties: request.properties,
            ..Default::default()
        };

        // TODO: create a token placeholder for the recipient with the expiration time etc.
        // this will then later be activated via the activation url

        let info = self.create(resource.into()).await?.0.try_into()?;
        Ok(info)
    }

    async fn delete_recipient(
        &self,
        request: DeleteRecipientRequest,
        context: RequestContext,
    ) -> Result<()> {
        self.check_required(&request, context.as_ref()).await?;
        self.delete(&request.resource()).await
    }

    async fn get_recipient(
        &self,
        request: GetRecipientRequest,
        context: RequestContext,
    ) -> Result<RecipientInfo> {
        self.check_required(&request, context.recipient()).await?;
        self.get(&request.resource()).await?.0.try_into()
    }

    async fn list_recipients(
        &self,
        request: ListRecipientsRequest,
        context: RequestContext,
    ) -> Result<ListRecipientsResponse> {
        self.check_required(&request, context.as_ref()).await?;
        let (mut resources, next_page_token) = self
            .list(
                &ObjectLabel::RecipientInfo,
                None,
                request.max_results.map(|v| v as usize),
                request.page_token,
            )
            .await?;
        process_resources(self, context.as_ref(), &Permission::Read, &mut resources).await?;
        Ok(ListRecipientsResponse {
            recipients: resources.into_iter().map(|r| r.try_into()).try_collect()?,
            next_page_token,
        })
    }

    async fn update_recipient(
        &self,
        _request: UpdateRecipientRequest,
        _context: RequestContext,
    ) -> Result<RecipientInfo> {
        // TODO: once we have token handling, we can update token expiration etc...
        todo!("update_recipient")
    }
}
