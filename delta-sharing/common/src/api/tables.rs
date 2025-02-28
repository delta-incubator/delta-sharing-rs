use delta_sharing_derive::rest_handlers;

use super::{RequestContext, SecuredAction};
use crate::models::tables::v1::*;
use crate::{Error, Permission, Recipient, ResourceIdent, ResourceName, ResourceRef, Result};

rest_handlers!(
    TablesHandler, "tables", [
        GetTableRequest, Table, Read, TableInfo with [
            full_name: path as String,
        ];
    ]
);

#[async_trait::async_trait]
pub trait TablesHandler: Send + Sync + 'static {
    /// Get a table.
    async fn get_table(
        &self,
        request: GetTableRequest,
        context: RequestContext,
    ) -> Result<TableInfo>;
}
