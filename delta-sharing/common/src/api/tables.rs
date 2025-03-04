use delta_sharing_derive::rest_handlers;

use super::{RequestContext, SecuredAction};
use crate::models::tables::v1::*;
use crate::{Error, Permission, Recipient, ResourceIdent, ResourceName, ResourceRef, Result};

rest_handlers!(
    TablesHandler, "tables", [
        ListTableSummariesRequest, Table, Read, ListTableSummariesResponse with [
            catalog_name: query as String,
            schema_name_pattern: query as Option<String>,
            table_name_pattern: query as Option<String>,
            include_manifest_capabilities: query as Option<bool>,
        ];
        ListTablesRequest, Table, Read, ListTablesResponse with [
            catalog_name: query as String,
            schema_name: query as String,
            include_delta_metadata: query as Option<bool>,
            omit_columns: query as Option<bool>,
            omit_properties: query as Option<bool>,
            omit_username: query as Option<bool>,
            include_browse: query as Option<bool>,
            include_manifest_capabilities: query as Option<bool>,
        ];
        GetTableRequest, Table, Read, TableInfo with [
            full_name: path as String,
            include_delta_metadata: query as Option<bool>,
            include_browse: query as Option<bool>,
            include_manifest_capabilities: query as Option<bool>,
        ];
        GetTableExistsRequest, Table, Read, GetTableExistsResponse with [
            full_name: path as String,
        ];
        DeleteTableRequest, Table, Write with [
            full_name: path as String,
        ];
    ]
);

#[async_trait::async_trait]
pub trait TablesHandler: Send + Sync + 'static {
    /// List table summaries.
    async fn list_table_summaries(
        &self,
        request: ListTableSummariesRequest,
        context: RequestContext,
    ) -> Result<ListTableSummariesResponse>;

    /// List tables.
    async fn list_tables(
        &self,
        request: ListTablesRequest,
        context: RequestContext,
    ) -> Result<ListTablesResponse>;

    /// Get a table.
    async fn get_table(
        &self,
        request: GetTableRequest,
        context: RequestContext,
    ) -> Result<TableInfo>;

    /// Check if a table exists.
    async fn get_table_exists(
        &self,
        request: GetTableExistsRequest,
        context: RequestContext,
    ) -> Result<GetTableExistsResponse>;

    /// Delete a table.
    async fn delete_table(
        &self,
        request: DeleteTableRequest,
        context: RequestContext,
    ) -> Result<()>;
}
