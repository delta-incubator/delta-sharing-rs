use delta_sharing_derive::rest_handlers;

use super::{RequestContext, SecuredAction};
use crate::models::external_locations::v1::*;
use crate::{Error, Permission, Recipient, ResourceIdent, ResourceName, ResourceRef, Result};

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
