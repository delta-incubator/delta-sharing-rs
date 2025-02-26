use delta_sharing_derive::rest_handlers;

use super::{RequestContext, SecuredAction};
use crate::models::recipients::v1::*;
use crate::{Error, Permission, Recipient, ResourceIdent, ResourceName, ResourceRef, Result};

rest_handlers!(
    RecipientsHandler, [
        CreateRecipientRequest, Recipient, Create, RecipientInfo;
        ListRecipientsRequest, Recipient, Read, ListRecipientsResponse;
        GetRecipientRequest, Recipient, Read, RecipientInfo with [
            name: path as String,
        ];
        UpdateRecipientRequest, Recipient, Manage, RecipientInfo with [
            name: path as String,
        ];
        DeleteRecipientRequest, Recipient, Manage with [
            name: path as String
        ];
    ]
);

#[async_trait::async_trait]
pub trait RecipientsHandler: Send + Sync + 'static {
    /// Create a new recipient.
    async fn create_recipient(
        &self,
        request: CreateRecipientRequest,
        context: RequestContext,
    ) -> Result<RecipientInfo>;

    /// Delete a recipient.
    async fn delete_recipient(
        &self,
        request: DeleteRecipientRequest,
        context: RequestContext,
    ) -> Result<()>;

    /// Get a recipient.
    async fn get_recipient(
        &self,
        request: GetRecipientRequest,
        context: RequestContext,
    ) -> Result<RecipientInfo>;

    /// List recipients.
    async fn list_recipients(
        &self,
        request: ListRecipientsRequest,
        context: RequestContext,
    ) -> Result<ListRecipientsResponse>;

    /// Update a recipient.
    async fn update_recipient(
        &self,
        request: UpdateRecipientRequest,
        context: RequestContext,
    ) -> Result<RecipientInfo>;
}
