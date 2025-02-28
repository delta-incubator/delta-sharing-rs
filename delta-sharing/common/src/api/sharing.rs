use delta_sharing_derive::rest_handlers;
use itertools::Itertools;

use super::{RequestContext, SecuredAction};
use crate::models::sharing::v1::*;
use crate::policy::{process_resources, Permission, Policy};
use crate::resources::{ResourceIdent, ResourceName, ResourceRef};
use crate::{AssociationLabel, Error, ObjectLabel, Recipient, Resource, ResourceStore, Result};

rest_handlers!(
    SharingDiscoveryHandler, "shares/schemas/tables", [
        ListSharesRequest, Share, Use, ListSharesResponse;
        GetShareRequest, Share, Use, Share with [
            name: path as String,
        ];
        ListSharingSchemasRequest, Share, Use, ListSharingSchemasResponse with [
            share: path as String,
        ];
        ListShareTablesRequest, Share, Use, ListShareTablesResponse with [
            name: path as String,
        ];
        ListSchemaTablesRequest, SharingSchema, Use, ListSchemaTablesResponse with [
            share: path as String,
            name: path as String,
        ];
    ]
);

#[async_trait::async_trait]
pub trait SharingDiscoveryHandler: Send + Sync + 'static {
    /// List all shares that the recipient is allowed to read.
    async fn list_shares(
        &self,
        request: ListSharesRequest,
        context: RequestContext,
    ) -> Result<ListSharesResponse>;

    /// Get a share by name.
    async fn get_share(&self, request: GetShareRequest, context: RequestContext) -> Result<Share>;

    /// List all schemas in a share.
    async fn list_sharing_schemas(
        &self,
        request: ListSharingSchemasRequest,
        context: RequestContext,
    ) -> Result<ListSharingSchemasResponse>;

    /// List all tables in a schema.
    async fn list_schema_tables(
        &self,
        request: ListSchemaTablesRequest,
        context: RequestContext,
    ) -> Result<ListSchemaTablesResponse>;

    /// List all tables in a share.
    async fn list_share_tables(
        &self,
        request: ListShareTablesRequest,
        context: RequestContext,
    ) -> Result<ListShareTablesResponse>;
}

rest_handlers!(
    SharingQueryHandler, "shares/schemas/tables", [
        GetTableVersionRequest, SharingTable, Read, GetTableVersionResponse with [
            share: path as String,
            schema: path as String,
            name: path as String,
            starting_timestamp: query as Option<String>
        ];
        GetTableMetadataRequest, SharingTable, Read, QueryResponse with [
            share: path as String,
            schema: path as String,
            name: path as String,
        ];
    ]
);

#[async_trait::async_trait]
pub trait SharingQueryHandler: Send + Sync + 'static {
    async fn get_table_version(
        &self,
        request: GetTableVersionRequest,
        context: RequestContext,
    ) -> Result<GetTableVersionResponse>;

    async fn get_table_metadata(
        &self,
        request: GetTableMetadataRequest,
        context: RequestContext,
    ) -> Result<QueryResponse>;
}

#[async_trait::async_trait]
impl<T: ResourceStore + Policy> SharingDiscoveryHandler for T {
    async fn list_shares(
        &self,
        request: ListSharesRequest,
        context: RequestContext,
    ) -> Result<ListSharesResponse> {
        self.check_required(&request, context.as_ref()).await?;
        let (mut resources, next_page_token) = self
            .list(
                &ObjectLabel::ShareInfo,
                None,
                request.max_results.map(|v| v as usize),
                request.page_token.clone(),
            )
            .await?;
        process_resources(self, context.as_ref(), &Permission::Read, &mut resources).await?;

        // if all resources gor filtered, but there are more pages, try again
        if resources.is_empty() && next_page_token.is_some() {
            return self.list_shares(request, context).await;
        }

        Ok(ListSharesResponse {
            items: resources.into_iter().map(|r| r.try_into()).try_collect()?,
            next_page_token,
        })
    }

    async fn get_share(&self, request: GetShareRequest, context: RequestContext) -> Result<Share> {
        self.check_required(&request, context.recipient()).await?;
        self.get(&request.resource()).await?.0.try_into()
    }

    async fn list_sharing_schemas(
        &self,
        request: ListSharingSchemasRequest,
        context: RequestContext,
    ) -> Result<ListSharingSchemasResponse> {
        self.check_required(&request, context.recipient()).await?;
        let (idents, next_page_token) = self
            .list_associations(
                &request.resource(),
                &AssociationLabel::ParentOf,
                Some(&ResourceIdent::SharingSchema(ResourceRef::Undefined)),
                request.max_results.map(|m| m as usize),
                request.page_token,
            )
            .await?;
        let (mut resources, _): (Vec<Resource>, Vec<ResourceRef>) =
            self.get_many(&idents).await?.into_iter().unzip();
        process_resources(self, context.as_ref(), &Permission::Read, &mut resources).await?;
        Ok(ListSharingSchemasResponse {
            items: resources.into_iter().map(|r| r.try_into()).try_collect()?,
            next_page_token,
        })
    }

    async fn list_schema_tables(
        &self,
        _request: ListSchemaTablesRequest,
        _context: RequestContext,
    ) -> Result<ListSchemaTablesResponse> {
        // Scaffold method body (implementation to come later)
        todo!()
    }

    async fn list_share_tables(
        &self,
        _request: ListShareTablesRequest,
        _context: RequestContext,
    ) -> Result<ListShareTablesResponse> {
        // Scaffold method body (implementation to come later)
        todo!()
    }
}
