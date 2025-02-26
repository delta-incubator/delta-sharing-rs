use delta_sharing_derive::rest_handlers;

use super::{RequestContext, SecuredAction};
use crate::models::schemas::v1::*;
use crate::{Error, Permission, Recipient, ResourceIdent, ResourceName, ResourceRef, Result};

rest_handlers!(
    SchemasHandler, [
        CreateSchemaRequest, Schema, Create, SchemaInfo;
        ListSchemasRequest, Catalog, Read, ListSchemasResponse with [
            catalog_name: query as String,
            include_browse: query as Option<bool>,
        ];
        GetSchemaRequest, Schema, Read, SchemaInfo with [
            full_name: path as String,
        ];
        UpdateSchemaRequest, Schema, Manage, SchemaInfo with [
            full_name: path as String,
        ];
        DeleteSchemaRequest, Schema, Manage with [
            full_name: path as String,
            force: query as Option<bool>,
        ];
    ]
);

#[async_trait::async_trait]
pub trait SchemasHandler: Send + Sync + 'static {
    /// Create a new schema.
    async fn create_schema(
        &self,
        request: CreateSchemaRequest,
        context: RequestContext,
    ) -> Result<SchemaInfo>;

    /// Delete a schema.
    async fn delete_schema(
        &self,
        request: DeleteSchemaRequest,
        context: RequestContext,
    ) -> Result<()>;

    /// Get a schema.
    async fn get_schema(
        &self,
        request: GetSchemaRequest,
        context: RequestContext,
    ) -> Result<SchemaInfo>;

    /// List schemas.
    async fn list_schemas(
        &self,
        request: ListSchemasRequest,
        context: RequestContext,
    ) -> Result<ListSchemasResponse>;

    /// Update a schema.
    async fn update_schema(
        &self,
        request: UpdateSchemaRequest,
        context: RequestContext,
    ) -> Result<SchemaInfo>;
}
