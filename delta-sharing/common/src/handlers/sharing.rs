use itertools::Itertools;

use super::ServerHandler;
use crate::api::{
    RequestContext, SharingDiscoveryHandler, SharingExtensionHandler, SharingQueryHandler,
};
use crate::models::sharing::v1::*;
use crate::policy::{process_resources, Permission};
use crate::{AssociationLabel, Resource, ResourceIdent, ResourceRef, Result, SecuredAction};

#[async_trait::async_trait]
impl SharingDiscoveryHandler for ServerHandler {
    async fn list_shares(
        &self,
        request: ListSharesRequest,
        context: RequestContext,
    ) -> Result<ListSharesResponse> {
        self.as_ref()
            .check_required(&request, context.as_ref())
            .await?;
        let (mut resources, next_page_token) = self
            .store
            .list(
                &ResourceIdent::share(ResourceRef::Undefined),
                request.max_results.map(|v| v as usize),
                request.page_token,
            )
            .await?;
        process_resources(self, context.as_ref(), &Permission::Read, &mut resources).await?;
        Ok(ListSharesResponse {
            items: resources.into_iter().map(|r| r.try_into()).try_collect()?,
            next_page_token,
        })
    }

    async fn get_share(&self, request: GetShareRequest, context: RequestContext) -> Result<Share> {
        self.as_ref()
            .check_required(&request, context.recipient())
            .await?;
        self.store.get(&request.resource()).await?.0.try_into()
    }

    async fn list_sharing_schemas(
        &self,
        request: ListSharingSchemasRequest,
        context: RequestContext,
    ) -> Result<ListSharingSchemasResponse> {
        self.as_ref()
            .check_required(&request, context.recipient())
            .await?;
        let (idents, next_page_token) = self
            .store
            .list_associations(
                &request.resource(),
                &AssociationLabel::ParentOf,
                Some(&ResourceIdent::SharingSchema(ResourceRef::Undefined)),
                request.max_results.map(|m| m as usize),
                request.page_token,
            )
            .await?;
        let (mut resources, _): (Vec<Resource>, Vec<ResourceRef>) =
            self.store.get_many(&idents).await?.into_iter().unzip();
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

#[async_trait::async_trait]
impl SharingExtensionHandler for ServerHandler {
    async fn create_share(
        &self,
        request: CreateShareRequest,
        context: RequestContext,
    ) -> Result<ShareInfo> {
        self.as_ref()
            .check_required(&request, context.recipient())
            .await?;
        let resource = ShareInfo {
            name: request.name,
            description: request.description,
            properties: request.properties,
            ..Default::default()
        };
        self.store.create(resource.into()).await?.0.try_into()
    }

    async fn delete_share(
        &self,
        request: DeleteShareRequest,
        context: RequestContext,
    ) -> Result<()> {
        self.as_ref()
            .check_required(&request, context.recipient())
            .await?;
        self.store.delete(&request.resource()).await
    }

    async fn create_sharing_schema(
        &self,
        request: CreateSharingSchemaRequest,
        context: RequestContext,
    ) -> Result<SharingSchemaInfo> {
        self.as_ref()
            .check_required(&request, context.recipient())
            .await?;
        todo!()
    }

    async fn delete_sharing_schema(
        &self,
        request: DeleteSharingSchemaRequest,
        context: RequestContext,
    ) -> Result<()> {
        self.as_ref()
            .check_required(&request, context.recipient())
            .await?;
        self.store.delete(&request.resource()).await
    }
}

#[async_trait::async_trait]
impl SharingQueryHandler for ServerHandler {
    async fn get_table_version(
        &self,
        request: GetTableVersionRequest,
        context: RequestContext,
    ) -> Result<GetTableVersionResponse> {
        self.as_ref()
            .check_required(&request, context.recipient())
            .await?;
        self.query.get_table_version(request).await
    }

    async fn get_table_metadata(
        &self,
        request: GetTableMetadataRequest,
        context: RequestContext,
    ) -> Result<QueryResponse> {
        self.as_ref()
            .check_required(&request, context.recipient())
            .await?;
        self.query.get_table_metadata(request).await
    }
}
