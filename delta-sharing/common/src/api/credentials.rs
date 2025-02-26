use delta_sharing_derive::rest_handlers;

use super::{RequestContext, SecuredAction};
use crate::models::credentials::v1::*;
use crate::{Error, Permission, Recipient, ResourceIdent, ResourceName, ResourceRef, Result};

rest_handlers!(
    CredentialsHandler,
    [
        ListCredentialsRequest, Credential, Read, ListCredentialsResponse;
        CreateCredentialRequest, Credential, Create, CredentialInfo;
        GetCredentialRequest, Credential, Read, CredentialInfo with [
            name: path as String,
        ];
        UpdateCredentialRequest, Credential, Manage, CredentialInfo with [
            name: path as String,
        ];
        DeleteCredentialRequest, Credential, Manage with [
            name: path as String,
        ];
    ]
);

#[async_trait::async_trait]
pub trait CredentialsHandler: Send + Sync + 'static {
    /// List credentials.
    async fn list_credentials(
        &self,
        request: ListCredentialsRequest,
        context: RequestContext,
    ) -> Result<ListCredentialsResponse>;

    /// Create a new credential.
    async fn create_credential(
        &self,
        request: CreateCredentialRequest,
        context: RequestContext,
    ) -> Result<CredentialInfo>;

    /// Get a credential.
    async fn get_credential(
        &self,
        request: GetCredentialRequest,
        context: RequestContext,
    ) -> Result<CredentialInfo>;

    /// Update a credential.
    async fn update_credential(
        &self,
        request: UpdateCredentialRequest,
        context: RequestContext,
    ) -> Result<CredentialInfo>;

    /// Delete a credential.
    async fn delete_credential(
        &self,
        request: DeleteCredentialRequest,
        context: RequestContext,
    ) -> Result<()>;
}
