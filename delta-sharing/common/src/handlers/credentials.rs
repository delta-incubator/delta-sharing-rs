use itertools::Itertools;

use crate::api::{CredentialsHandler, RequestContext};
use crate::models::credentials::v1::*;
use crate::{ObjectLabel, Policy, ResourceStore, Result, SecuredAction};

#[async_trait::async_trait]
impl<T: ResourceStore + Policy> CredentialsHandler for T {
    async fn create_credential(
        &self,
        request: CreateCredentialRequest,
        context: RequestContext,
    ) -> Result<Credential> {
        self.check_required(&request, context.recipient()).await?;
        todo!()
    }

    async fn get_credential(
        &self,
        request: GetCredentialRequest,
        context: RequestContext,
    ) -> Result<Credential> {
        self.check_required(&request, context.recipient()).await?;
        self.get(&request.resource()).await?.0.try_into()
    }

    async fn delete_credential(
        &self,
        request: DeleteCredentialRequest,
        context: RequestContext,
    ) -> Result<()> {
        self.check_required(&request, context.recipient()).await?;
        self.delete(&request.resource()).await
    }

    async fn create_storage_location(
        &self,
        request: CreateStorageLocationRequest,
        context: RequestContext,
    ) -> Result<StorageLocation> {
        self.check_required(&request, context.recipient()).await?;
        let resource = StorageLocation {
            name: request.name,
            url: request.url,
            description: request.description,
            r#type: request.r#type,
            properties: request.properties,
            credential: request.credential,
            ..Default::default()
        };
        self.create(resource.into()).await?.0.try_into()
    }

    async fn get_storage_location(
        &self,
        request: GetStorageLocationRequest,
        context: RequestContext,
    ) -> Result<StorageLocation> {
        self.check_required(&request, context.recipient()).await?;
        self.get(&request.resource()).await?.0.try_into()
    }

    async fn delete_storage_location(
        &self,
        request: DeleteStorageLocationRequest,
        context: RequestContext,
    ) -> Result<()> {
        self.check_required(&request, context.recipient()).await?;
        self.delete(&request.resource()).await
    }

    async fn list_storage_locations(
        &self,
        request: ListStorageLocationsRequest,
        context: RequestContext,
    ) -> Result<ListStorageLocationsResponse> {
        self.check_required(&request, context.recipient()).await?;
        let (resources, next_page_token) = self
            .list(
                &ObjectLabel::StorageLocation,
                None,
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
