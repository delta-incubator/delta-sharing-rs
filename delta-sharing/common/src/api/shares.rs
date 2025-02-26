use delta_sharing_derive::rest_handlers;

use super::{RequestContext, SecuredAction};
use crate::models::shares::v1::*;
use crate::{Error, Permission, Recipient, ResourceIdent, ResourceName, ResourceRef, Result};

rest_handlers!(
    SharesHandler, [
        CreateShareRequest, Schema, Create, ShareInfo;
        ListSharesRequest, Catalog, Read, ListSharesResponse;
        GetShareRequest, Schema, Read, ShareInfo with [
            name: path as String,
        ];
        UpdateShareRequest, Schema, Manage, ShareInfo with [
            name: path as String,
        ];
        DeleteShareRequest, Schema, Manage with [
            name: path as String
        ];
    ]
);

#[async_trait::async_trait]
pub trait SharesHandler: Send + Sync + 'static {
    /// Create a new share.
    async fn create_share(
        &self,
        request: CreateShareRequest,
        context: RequestContext,
    ) -> Result<ShareInfo>;

    /// Delete a share.
    async fn delete_share(
        &self,
        request: DeleteShareRequest,
        context: RequestContext,
    ) -> Result<()>;

    /// Get a share.
    async fn get_share(
        &self,
        request: GetShareRequest,
        context: RequestContext,
    ) -> Result<ShareInfo>;

    /// List shares.
    async fn list_shares(
        &self,
        request: ListSharesRequest,
        context: RequestContext,
    ) -> Result<ListSharesResponse>;

    /// Update a share.
    async fn update_share(
        &self,
        request: UpdateShareRequest,
        context: RequestContext,
    ) -> Result<ShareInfo>;
}
