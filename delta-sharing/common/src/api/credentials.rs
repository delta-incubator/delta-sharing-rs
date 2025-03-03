use delta_sharing_derive::rest_handlers;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use super::{RequestContext, SecuredAction};
use crate::models::credentials::v1::*;
use crate::policy::{process_resources, Permission, Policy};
use crate::resources::{ResourceIdent, ResourceName, ResourceRef};
use crate::{Error, ObjectLabel, Recipient, ResourceExt, ResourceStore, Result, SecretManager};

rest_handlers!(
    CredentialsHandler, "credentials",
    [
        ListCredentialsRequest, Credential, Read, ListCredentialsResponse with [
            purpose: query as Option<i32>,
        ];
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

#[derive(Clone, Serialize, Deserialize)]
struct CredentialContainer {
    pub azure_sp: Option<AzureServicePrincipal>,
    pub azure_msi: Option<AzureManagedIdentity>,
    pub azure_key: Option<AzureStorageKey>,
}

impl CredentialContainer {
    fn from_get(cred: create_credential_request::Credential) -> Self {
        match cred {
            create_credential_request::Credential::AzureServicePrincipal(azure_sp) => Self {
                azure_sp: Some(azure_sp),
                azure_msi: None,
                azure_key: None,
            },
            create_credential_request::Credential::AzureManagedIdentity(azure_msi) => Self {
                azure_sp: None,
                azure_msi: Some(azure_msi),
                azure_key: None,
            },
            create_credential_request::Credential::AzureStorageKey(azure_key) => Self {
                azure_sp: None,
                azure_msi: None,
                azure_key: Some(azure_key),
            },
        }
    }

    fn from_update(cred: update_credential_request::Credential) -> Self {
        match cred {
            update_credential_request::Credential::AzureServicePrincipal(azure_sp) => Self {
                azure_sp: Some(azure_sp),
                azure_msi: None,
                azure_key: None,
            },
            update_credential_request::Credential::AzureManagedIdentity(azure_msi) => Self {
                azure_sp: None,
                azure_msi: Some(azure_msi),
                azure_key: None,
            },
            update_credential_request::Credential::AzureStorageKey(azure_key) => Self {
                azure_sp: None,
                azure_msi: None,
                azure_key: Some(azure_key),
            },
        }
    }

    fn into_cred(self) -> Result<credential_info::Credential> {
        if let Some(azure_sp) = self.azure_sp {
            Ok(credential_info::Credential::AzureServicePrincipal(azure_sp))
        } else if let Some(azure_msi) = self.azure_msi {
            Ok(credential_info::Credential::AzureManagedIdentity(azure_msi))
        } else {
            Err(Error::invalid_argument("credential is required"))
        }
    }

    fn to_vec(&self) -> Result<Vec<u8>> {
        Ok(serde_json::to_vec(self)?)
    }
}

#[async_trait::async_trait]
impl<T: ResourceStore + Policy + SecretManager> CredentialsHandler for T {
    async fn list_credentials(
        &self,
        request: ListCredentialsRequest,
        context: RequestContext,
    ) -> Result<ListCredentialsResponse> {
        self.check_required(&request, context.as_ref()).await?;
        let (mut resources, next_page_token) = self
            .list(
                &ObjectLabel::CredentialInfo,
                None,
                request.max_results.map(|v| v as usize),
                request.page_token,
            )
            .await?;
        process_resources(self, context.as_ref(), &Permission::Read, &mut resources).await?;
        Ok(ListCredentialsResponse {
            credentials: resources.into_iter().map(|r| r.try_into()).try_collect()?,
            next_page_token,
        })
    }
    async fn create_credential(
        &self,
        request: CreateCredentialRequest,
        context: RequestContext,
    ) -> Result<CredentialInfo> {
        self.check_required(&request, context.recipient()).await?;
        let secret = request
            .credential
            .ok_or_else(|| Error::invalid_argument("credential is required"))?;
        self.create_secret(
            &request.name,
            CredentialContainer::from_get(secret).to_vec()?.into(),
        )
        .await?;
        let cred = CredentialInfo {
            name: request.name.clone(),
            full_name: Some(request.name),
            comment: request.comment,
            purpose: request.purpose,
            read_only: request.read_only.unwrap_or(false),
            used_for_managed_storage: false,
            id: "".to_string(),
            created_at: None,
            updated_at: None,
            credential: None,
            owner: None,
            created_by: None,
            updated_by: None,
        };
        self.create(cred.into()).await?.0.try_into()
    }

    async fn get_credential(
        &self,
        request: GetCredentialRequest,
        context: RequestContext,
    ) -> Result<CredentialInfo> {
        self.check_required(&request, context.recipient()).await?;
        let mut cred: CredentialInfo = self.get(&request.resource()).await?.0.try_into()?;
        let (_, secret_data) = self.get_secret(&cred.name).await?;
        let secret: CredentialContainer = serde_json::from_slice(&secret_data)?;
        cred.credential = Some(secret.into_cred()?);
        Ok(cred)
    }

    async fn update_credential(
        &self,
        request: UpdateCredentialRequest,
        context: RequestContext,
    ) -> Result<CredentialInfo> {
        self.check_required(&request, context.recipient()).await?;
        if let Some(credential) = request.credential {
            self.update_secret(
                &request.name,
                CredentialContainer::from_update(credential.clone())
                    .to_vec()?
                    .into(),
            )
            .await?;
        }
        let curr = self
            .get_credential(
                GetCredentialRequest {
                    name: request.name.clone(),
                },
                context.clone(),
            )
            .await?;
        let cred = CredentialInfo {
            name: request.name.clone(),
            full_name: Some(request.name),
            comment: request.comment,
            purpose: curr.purpose,
            read_only: request.read_only.unwrap_or(false),
            used_for_managed_storage: false,
            id: "".to_string(),
            created_at: None,
            updated_at: None,
            credential: None,
            owner: None,
            created_by: None,
            updated_by: None,
        };
        self.update(&curr.resource_ident(), cred.into())
            .await?
            .0
            .try_into()
    }

    async fn delete_credential(
        &self,
        request: DeleteCredentialRequest,
        context: RequestContext,
    ) -> Result<()> {
        self.check_required(&request, context.recipient()).await?;
        self.delete_secret(&request.name).await?;
        self.delete(&request.resource()).await
    }
}
