use delta_sharing_derive::rest_handlers;

use super::{RequestContext, SecuredAction};
use crate::models::credentials::v1::*;
use crate::{Error, Permission, Recipient, ResourceIdent, ResourceName, ResourceRef, Result};

rest_handlers!(
    CredentialsHandler,
    [
        CreateCredentialRequest, Credential, Create, Credential;
        GetCredentialRequest, Credential, Read, Credential with [
            name: path as String,
        ];
        DeleteCredentialRequest, Credential, Manage with [
            name: path as String,
        ];
    ]
);

#[async_trait::async_trait]
pub trait CredentialsHandler: Send + Sync + 'static {
    /// Create a new credential.
    async fn create_credential(
        &self,
        request: CreateCredentialRequest,
        context: RequestContext,
    ) -> Result<Credential>;

    /// Delete a credential.
    async fn delete_credential(
        &self,
        request: DeleteCredentialRequest,
        context: RequestContext,
    ) -> Result<()>;

    /// Get a credential.
    async fn get_credential(
        &self,
        request: GetCredentialRequest,
        context: RequestContext,
    ) -> Result<Credential>;
}
