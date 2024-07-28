use std::sync::Arc;

use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::models::{v1::*, TableRef};
use crate::{DiscoveryHandler, Recipient, TableLocationResover};

#[cfg(feature = "profiles")]
pub type DefaultInMemoryHandler = InMemoryHandler;
#[cfg(not(feature = "profiles"))]
pub type DefaultInMemoryHandler = InMemoryHandler<()>;

#[derive(Debug, Serialize, Deserialize)]
pub struct TableConfig {
    pub name: String,
    pub location: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaConfig {
    pub name: String,
    pub table_refs: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShareConfig {
    pub name: String,
    pub schema_refs: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InMemoryConfig {
    pub shares: Vec<ShareConfig>,
    pub schemas: Vec<SchemaConfig>,
    pub tables: Vec<TableConfig>,
}

pub struct InMemoryHandler {
    // The data in memory
    shares: Arc<DashMap<String, Vec<String>>>,
    schemas: Arc<DashMap<String, Vec<String>>>,
    tables: Arc<DashMap<String, TableConfig>>,
}

impl InMemoryHandler {
    pub fn new(config: InMemoryConfig) -> Self {
        let shares = Arc::new(DashMap::new());
        let schemas = Arc::new(DashMap::new());
        let tables = Arc::new(DashMap::new());

        for share in config.shares {
            shares.insert(share.name, share.schema_refs);
        }

        for schema in config.schemas {
            schemas.insert(schema.name, schema.table_refs);
        }

        for table in config.tables {
            tables.insert(table.name.clone(), table);
        }

        Self {
            shares,
            schemas,
            tables,
        }
    }
}

#[async_trait::async_trait]
impl DiscoveryHandler for InMemoryHandler {
    async fn list_shares(
        &self,
        _request: ListSharesRequest,
        _recipient: &Recipient,
    ) -> Result<ListSharesResponse> {
        let shares = self
            .shares
            .iter()
            .map(|share| {
                let id = Uuid::new_v5(&Uuid::NAMESPACE_OID, share.key().as_bytes());
                Share {
                    id: Some(id.into()),
                    name: share.key().clone(),
                }
            })
            .collect();
        Ok(ListSharesResponse {
            items: shares,
            next_page_token: None,
        })
    }

    async fn get_share(&self, request: GetShareRequest) -> Result<GetShareResponse> {
        if self.shares.contains_key(&request.share) {
            let id = Uuid::new_v5(&Uuid::NAMESPACE_OID, request.share.as_bytes());
            Ok(GetShareResponse {
                share: Some(Share {
                    id: Some(id.to_string()),
                    name: request.share,
                }),
            })
        } else {
            Err(Error::NotFound)
        }
    }

    async fn list_schemas(&self, request: ListSchemasRequest) -> Result<ListSchemasResponse> {
        match self.shares.get(&request.share) {
            Some(schema_refs) => {
                let schemas = schema_refs
                    .iter()
                    .map(|schema_ref| Schema {
                        name: schema_ref.clone(),
                        share: request.share.clone(),
                    })
                    .collect();
                Ok(ListSchemasResponse {
                    items: schemas,
                    next_page_token: None,
                })
            }
            None => Err(Error::NotFound),
        }
    }

    async fn list_schema_tables(
        &self,
        request: ListSchemaTablesRequest,
    ) -> Result<ListSchemaTablesResponse> {
        let schema_refs = self.shares.get(&request.share).ok_or(Error::NotFound)?;
        if !schema_refs.contains(&request.schema) {
            return Err(Error::NotFound);
        }
        let share_id = Uuid::new_v5(&Uuid::NAMESPACE_OID, request.share.as_bytes());
        match self.schemas.get(&request.schema) {
            Some(tables) => {
                let tables = tables
                    .iter()
                    .flat_map(|table_ref| {
                        self.tables.get(table_ref).map(|v| Table {
                            id: Some(Uuid::new_v5(&share_id, v.name.as_bytes()).to_string()),
                            name: v.name.clone(),
                            share: request.share.clone(),
                            schema: request.schema.clone(),
                            share_id: Some(share_id.to_string()),
                        })
                    })
                    .collect();
                Ok(ListSchemaTablesResponse {
                    items: tables,
                    next_page_token: None,
                })
            }
            None => Err(crate::error::Error::NotFound),
        }
    }

    async fn list_share_tables(
        &self,
        request: ListShareTablesRequest,
    ) -> Result<ListShareTablesResponse> {
        let share_id = Uuid::new_v5(&Uuid::NAMESPACE_OID, request.share.as_bytes());
        match self.shares.get(&request.share) {
            Some(schema_refs) => {
                let tables = schema_refs
                    .iter()
                    .flat_map(|schema_ref| {
                        self.schemas.get(schema_ref).map(|v| {
                            v.iter()
                                .flat_map(|table_ref| {
                                    self.tables.get(table_ref).map(|v| Table {
                                        id: Some(
                                            Uuid::new_v5(&share_id, v.name.as_bytes()).to_string(),
                                        ),
                                        name: v.name.clone(),
                                        share: request.share.clone(),
                                        schema: schema_ref.clone(),
                                        share_id: Some(share_id.to_string()),
                                    })
                                })
                                .collect::<Vec<_>>()
                        })
                    })
                    .flatten()
                    .collect();
                Ok(ListShareTablesResponse {
                    items: tables,
                    next_page_token: None,
                })
            }
            None => Err(Error::NotFound),
        }
    }
}

#[async_trait::async_trait]
impl TableLocationResover for InMemoryHandler {
    async fn resolve(&self, table_ref: &TableRef) -> Result<url::Url> {
        let Some(schemas) = self.shares.get(&table_ref.share) else {
            return Err(Error::NotFound);
        };
        if !schemas.contains(&table_ref.schema) {
            return Err(Error::NotFound);
        }
        let Some(tables) = self.schemas.get(&table_ref.schema) else {
            return Err(Error::NotFound);
        };
        if !tables.contains(&table_ref.table) {
            return Err(Error::NotFound);
        }
        let table = self.tables.get(&table_ref.table).ok_or(Error::NotFound)?;
        Ok(url::Url::parse(&table.location)
            .map_err(|_| Error::InvalidTableLocation(table.location.clone()))?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_in_memory_handler() {
        let config = InMemoryConfig {
            shares: vec![ShareConfig {
                name: "share1".to_string(),
                schema_refs: vec!["schema1".to_string()],
            }],
            schemas: vec![SchemaConfig {
                name: "schema1".to_string(),
                table_refs: vec!["table1".to_string()],
            }],
            tables: vec![TableConfig {
                name: "table1".to_string(),
                location: "file:///tmp".to_string(),
            }],
        };
        let handler = DefaultInMemoryHandler::new(config);
        let recipient = &Recipient(bytes::Bytes::new());

        let shares = handler
            .list_shares(ListSharesRequest::default(), recipient)
            .await
            .unwrap();
        assert_eq!(shares.items.len(), 1);
        assert_eq!(shares.items[0].name, "share1");

        let share = handler
            .get_share(GetShareRequest {
                share: "share1".to_string(),
            })
            .await
            .unwrap();
        assert_eq!(share.share.unwrap().name, "share1");

        let schemas = handler
            .list_schemas(ListSchemasRequest {
                share: "share1".to_string(),
                pagination: None,
            })
            .await
            .unwrap();
        assert_eq!(schemas.items.len(), 1);
        assert_eq!(schemas.items[0].name, "schema1");

        let tables = handler
            .list_schema_tables(ListSchemaTablesRequest {
                share: "share1".to_string(),
                schema: "schema1".to_string(),
                pagination: None,
            })
            .await
            .unwrap();
        assert_eq!(tables.items.len(), 1);
        assert_eq!(tables.items[0].name, "table1");

        let tables = handler
            .list_share_tables(ListShareTablesRequest {
                share: "share1".to_string(),
                pagination: None,
            })
            .await
            .unwrap();
        assert_eq!(tables.items.len(), 1);
        assert_eq!(tables.items[0].name, "table1");
    }
}
