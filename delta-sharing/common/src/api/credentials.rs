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
        CreateStorageLocationRequest, StorageLocation, Create, StorageLocation;
        ListStorageLocationsRequest, StorageLocation, Read, ListStorageLocationsResponse;
        GetStorageLocationRequest, StorageLocation, Read, StorageLocation with [
            name: path as String,
        ];
        DeleteStorageLocationRequest, StorageLocation, Manage with [
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

    /// Create a new storage location.
    async fn create_storage_location(
        &self,
        request: CreateStorageLocationRequest,
        context: RequestContext,
    ) -> Result<StorageLocation>;

    /// Delete a storage location.
    async fn delete_storage_location(
        &self,
        request: DeleteStorageLocationRequest,
        context: RequestContext,
    ) -> Result<()>;

    /// Get a storage location.
    async fn get_storage_location(
        &self,
        request: GetStorageLocationRequest,
        context: RequestContext,
    ) -> Result<StorageLocation>;

    /// List storage locations.
    async fn list_storage_locations(
        &self,
        request: ListStorageLocationsRequest,
        context: RequestContext,
    ) -> Result<ListStorageLocationsResponse>;
}
