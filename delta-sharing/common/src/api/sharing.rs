use delta_sharing_derive::rest_handlers;

use super::{RequestContext, SecuredAction};
use crate::models::sharing::v1::*;
use crate::{Error, Permission, Recipient, ResourceIdent, ResourceName, ResourceRef, Result};

rest_handlers!(
    SharingDiscoveryHandler, [
        ListSharesRequest, Share, Read, ListSharesResponse;
        GetShareRequest, Share, Read, Share with [
            name: path as String,
        ];
        ListSharingSchemasRequest, Share, Read, ListSharingSchemasResponse with [
            share: path as String,
        ];
        ListShareTablesRequest, Share, Read, ListShareTablesResponse with [
            name: path as String,
        ];
        ListSchemaTablesRequest, SharingSchema, Read, ListSchemaTablesResponse with [
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
    SharingExtensionHandler, [
        CreateShareRequest, Share, Create, ShareInfo;
        DeleteShareRequest, Share, Manage with [
            name: path as String,
            force: query as Option<bool>
        ];
        CreateSharingSchemaRequest, SharingSchema, Create, SharingSchemaInfo with [
            share: path as String,
        ];
        DeleteSharingSchemaRequest, SharingSchema, Manage with [
            share: path as String,
            name: path as String,
        ];
    ]
);

#[async_trait::async_trait]
pub trait SharingExtensionHandler: Send + Sync + 'static {
    /// Create a share.
    async fn create_share(
        &self,
        request: CreateShareRequest,
        context: RequestContext,
    ) -> Result<ShareInfo>;

    /// Delete a share.
    async fn delete_share(
        &self,
        request: DeleteShareRequest,
        context: RequestContext,
    ) -> Result<()>;

    /// Create a schema.
    async fn create_sharing_schema(
        &self,
        request: CreateSharingSchemaRequest,
        context: RequestContext,
    ) -> Result<SharingSchemaInfo>;

    /// Delete a schema.
    async fn delete_sharing_schema(
        &self,
        request: DeleteSharingSchemaRequest,
        context: RequestContext,
    ) -> Result<()>;
}

rest_handlers!(
    SharingQueryHandler, [
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
