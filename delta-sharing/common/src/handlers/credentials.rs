use itertools::Itertools;

use super::ServerHandler;
use crate::api::{CredentialsHandler, RequestContext};
use crate::models::credentials::v1::*;
use crate::{Error, ResourceIdent, ResourceRef, Result, SecuredAction};

#[async_trait::async_trait]
impl CredentialsHandler for ServerHandler {
    async fn create_credential(
        &self,
        request: CreateCredentialRequest,
        context: RequestContext,
    ) -> Result<Credential> {
        self.policy
            .check_required(&request, context.recipient())
            .await?;
        todo!("create_credential")
    }

    async fn get_credential(
        &self,
        request: GetCredentialRequest,
        context: RequestContext,
    ) -> Result<Credential> {
        self.policy
            .check_required(&request, context.recipient())
            .await?;
        self.store.get(&request.resource()).await?.0.try_into()
    }

    async fn delete_credential(
        &self,
        request: DeleteCredentialRequest,
        context: RequestContext,
    ) -> Result<()> {
        self.policy
            .check_required(&request, context.recipient())
            .await?;
        self.store.delete(&request.resource()).await
    }

    async fn create_storage_location(
        &self,
        request: CreateStorageLocationRequest,
        context: RequestContext,
    ) -> Result<StorageLocation> {
        self.policy
            .check_required(&request, context.recipient())
            .await?;
        let Some(resource) = request.location else {
            return Err(Error::invalid_argument("storage location is required"));
        };
        self.store.create(resource.into()).await?.0.try_into()
    }

    async fn get_storage_location(
        &self,
        request: GetStorageLocationRequest,
        context: RequestContext,
    ) -> Result<StorageLocation> {
        self.policy
            .check_required(&request, context.recipient())
            .await?;
        self.store.get(&request.resource()).await?.0.try_into()
    }

    async fn delete_storage_location(
        &self,
        request: DeleteStorageLocationRequest,
        context: RequestContext,
    ) -> Result<()> {
        self.policy
            .check_required(&request, context.recipient())
            .await?;
        self.store.delete(&request.resource()).await
    }

    async fn list_storage_locations(
        &self,
        request: ListStorageLocationsRequest,
        context: RequestContext,
    ) -> Result<ListStorageLocationsResponse> {
        self.policy
            .check_required(&request, context.recipient())
            .await?;
        let (resources, next_page_token) = self
            .store
            .list(
                &ResourceIdent::storage_location(ResourceRef::Undefined),
                request.max_results.map(|v| v as usize),
                request.page_token,
            )
            .await?;
        Ok(ListStorageLocationsResponse {
            storage_locations: resources.into_iter().map(|l| l.try_into()).try_collect()?,
            next_page_token,
        })
    }
}
