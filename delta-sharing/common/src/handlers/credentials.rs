use crate::api::{CredentialsHandler, RequestContext};
use crate::models::credentials::v1::*;
use crate::{Policy, ResourceStore, Result, SecuredAction};

#[async_trait::async_trait]
impl<T: ResourceStore + Policy> CredentialsHandler for T {
    async fn list_credentials(
        &self,
        request: ListCredentialsRequest,
        context: RequestContext,
    ) -> Result<ListCredentialsResponse> {
        self.check_required(&request, context.recipient()).await?;
        todo!()
    }
    async fn create_credential(
        &self,
        request: CreateCredentialRequest,
        context: RequestContext,
    ) -> Result<CredentialInfo> {
        self.check_required(&request, context.recipient()).await?;
        todo!()
    }

    async fn get_credential(
        &self,
        request: GetCredentialRequest,
        context: RequestContext,
    ) -> Result<CredentialInfo> {
        self.check_required(&request, context.recipient()).await?;
        self.get(&request.resource()).await?.0.try_into()
    }

    async fn update_credential(
        &self,
        request: UpdateCredentialRequest,
        context: RequestContext,
    ) -> Result<CredentialInfo> {
        self.check_required(&request, context.recipient()).await?;
        todo!()
    }

    async fn delete_credential(
        &self,
        request: DeleteCredentialRequest,
        context: RequestContext,
    ) -> Result<()> {
        self.check_required(&request, context.recipient()).await?;
        self.delete(&request.resource()).await
    }
}
