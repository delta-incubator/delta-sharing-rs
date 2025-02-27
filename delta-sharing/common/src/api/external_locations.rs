use delta_sharing_derive::rest_handlers;
use itertools::Itertools;

use super::{RequestContext, SecuredAction};
use crate::models::external_locations::v1::*;
use crate::policy::{process_resources, Permission, Policy};
use crate::resources::{ResourceIdent, ResourceName, ResourceRef};
use crate::{Error, ObjectLabel, Recipient, ResourceExt, ResourceStore, Result};

rest_handlers!(
    ExternalLocationsHandler,
    [
        CreateExternalLocationRequest, ExternalLocation, Create, ExternalLocationInfo;
        ListExternalLocationsRequest, ExternalLocation, Read, ListExternalLocationsResponse with [
            include_browse: query as Option<bool>,
        ];
        GetExternalLocationRequest, ExternalLocation, Read, ExternalLocationInfo with [
            name: path as String,
        ];
        UpdateExternalLocationRequest, ExternalLocation, Manage, ExternalLocationInfo with [
            name: path as String,
        ];
        DeleteExternalLocationRequest, ExternalLocation, Manage with [
            name: path as String,
            force: query as Option<bool>,
        ];
    ]
);

#[async_trait::async_trait]
pub trait ExternalLocationsHandler: Send + Sync + 'static {
    /// Create a new external location.
    async fn create_external_location(
        &self,
        request: CreateExternalLocationRequest,
        context: RequestContext,
    ) -> Result<ExternalLocationInfo>;

    /// Delete an external location.
    async fn delete_external_location(
        &self,
        request: DeleteExternalLocationRequest,
        context: RequestContext,
    ) -> Result<()>;

    /// Get an external location.
    async fn get_external_location(
        &self,
        request: GetExternalLocationRequest,
        context: RequestContext,
    ) -> Result<ExternalLocationInfo>;

    /// List external locations.
    async fn list_external_locations(
        &self,
        request: ListExternalLocationsRequest,
        context: RequestContext,
    ) -> Result<ListExternalLocationsResponse>;

    /// Update an external location.
    async fn update_external_location(
        &self,
        request: UpdateExternalLocationRequest,
        context: RequestContext,
    ) -> Result<ExternalLocationInfo>;
}

#[async_trait::async_trait]
impl<T: ResourceStore + Policy> ExternalLocationsHandler for T {
    async fn create_external_location(
        &self,
        request: CreateExternalLocationRequest,
        context: RequestContext,
    ) -> Result<ExternalLocationInfo> {
        self.check_required(&request, context.as_ref()).await?;
        let mut resource = ExternalLocationInfo {
            name: request.name,
            url: request.url,
            credential_name: request.credential_name,
            read_only: request.read_only.unwrap_or(false),
            comment: request.comment,
            ..Default::default()
        };
        let cred_ident =
            ResourceIdent::Credential(ResourceName::from_naive_str_split(&resource.name).into());
        let (_credential, credential_ref) = self.get(&cred_ident).await?;
        if let ResourceRef::Uuid(uuid) = credential_ref {
            resource.credential_id = uuid.hyphenated().to_string();
        }

        // TODO: validate we can access the url with the provide credential

        let info = self.create(resource.into()).await?.0.try_into()?;
        Ok(info)
    }

    async fn delete_external_location(
        &self,
        request: DeleteExternalLocationRequest,
        context: RequestContext,
    ) -> Result<()> {
        self.check_required(&request, context.as_ref()).await?;
        // TODO: check if the location is used by any resources
        self.delete(&request.resource()).await
    }

    async fn get_external_location(
        &self,
        request: GetExternalLocationRequest,
        context: RequestContext,
    ) -> Result<ExternalLocationInfo> {
        self.check_required(&request, context.recipient()).await?;

        // TODO: populate relation fields (updated_* etc.)

        self.get(&request.resource()).await?.0.try_into()
    }

    async fn list_external_locations(
        &self,
        request: ListExternalLocationsRequest,
        context: RequestContext,
    ) -> Result<ListExternalLocationsResponse> {
        self.check_required(&request, context.recipient()).await?;
        let (mut resources, next_page_token) = self
            .list(
                &ObjectLabel::CatalogInfo,
                None,
                request.max_results.map(|v| v as usize),
                request.page_token,
            )
            .await?;
        process_resources(self, context.as_ref(), &Permission::Read, &mut resources).await?;
        Ok(ListExternalLocationsResponse {
            external_locations: resources.into_iter().map(|r| r.try_into()).try_collect()?,
            next_page_token,
        })
    }

    async fn update_external_location(
        &self,
        request: UpdateExternalLocationRequest,
        context: RequestContext,
    ) -> Result<ExternalLocationInfo> {
        self.check_required(&request, context.as_ref()).await?;

        let (current, _) = self.get(&request.resource()).await?;
        let curr_ident = current.resource_ident();
        let mut current: ExternalLocationInfo = current.try_into()?;

        if let Some(name) = request.new_name {
            current.name = name;
        }
        if let Some(url) = request.url {
            current.url = url;
        }
        if let Some(credential_name) = request.credential_name {
            current.credential_name = credential_name;
        }
        if let Some(read_only) = request.read_only {
            current.read_only = read_only;
        }
        if let Some(comment) = request.comment {
            current.comment = Some(comment);
        }

        // TODO:
        // - add update_* relations
        // - update owner if necessary

        self.update(&curr_ident, current.into()).await?.0.try_into()
    }
}
