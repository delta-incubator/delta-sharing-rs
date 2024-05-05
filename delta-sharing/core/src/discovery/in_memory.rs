use std::sync::Arc;

use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{DiscoveryHandler, TableLocationResover};
use crate::error::{Error, Result};
use crate::policies::DeltaRecipient;
use crate::types as t;

pub type DefaultInMemoryHandler = InMemoryHandler<DeltaRecipient>;

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
pub struct Config {
    pub shares: Vec<ShareConfig>,
    pub schemas: Vec<SchemaConfig>,
    pub tables: Vec<TableConfig>,
}

pub struct InMemoryHandler<T: Send + Sync> {
    // The data in memory
    shares: Arc<DashMap<String, Vec<String>>>,
    schemas: Arc<DashMap<String, Vec<String>>>,
    tables: Arc<DashMap<String, TableConfig>>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Send + Sync> InMemoryHandler<T> {
    pub fn new(config: Config) -> Self {
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
            _phantom: std::marker::PhantomData,
        }
    }
}

#[async_trait::async_trait]
impl<T: Send + Sync> DiscoveryHandler for InMemoryHandler<T> {
    type Recipient = T;

    async fn list_shares(
        &self,
        _request: t::ListSharesRequest,
        _recipient: Self::Recipient,
    ) -> Result<t::ListSharesResponse> {
        let shares = self
            .shares
            .iter()
            .map(|share| {
                let id = Uuid::new_v5(&Uuid::NAMESPACE_OID, share.key().as_bytes());
                t::Share {
                    id: Some(id.to_string()),
                    name: share.key().clone(),
                }
            })
            .collect();
        Ok(t::ListSharesResponse {
            items: shares,
            next_page_token: None,
        })
    }

    async fn get_share(&self, request: t::GetShareRequest) -> Result<t::GetShareResponse> {
        if self.shares.contains_key(&request.share) {
            let id = Uuid::new_v5(&Uuid::NAMESPACE_OID, request.share.as_bytes());
            Ok(t::GetShareResponse {
                share: Some(t::Share {
                    id: Some(id.to_string()),
                    name: request.share,
                }),
            })
        } else {
            Err(Error::NotFound)
        }
    }

    async fn list_schemas(&self, request: t::ListSchemasRequest) -> Result<t::ListSchemasResponse> {
        match self.shares.get(&request.share) {
            Some(schema_refs) => {
                let schemas = schema_refs
                    .iter()
                    .map(|schema_ref| t::Schema {
                        name: schema_ref.clone(),
                        share: request.share.clone(),
                    })
                    .collect();
                Ok(t::ListSchemasResponse {
                    items: schemas,
                    next_page_token: None,
                })
            }
            None => Err(Error::NotFound),
        }
    }

    async fn list_schema_tables(
        &self,
        request: t::ListSchemaTablesRequest,
    ) -> Result<t::ListSchemaTablesResponse> {
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
                        self.tables.get(table_ref).map(|v| t::Table {
                            id: Some(Uuid::new_v5(&share_id, v.name.as_bytes()).to_string()),
                            name: v.name.clone(),
                            share: request.share.clone(),
                            schema: request.schema.clone(),
                            share_id: Some(share_id.to_string()),
                        })
                    })
                    .collect();
                Ok(t::ListSchemaTablesResponse {
                    items: tables,
                    next_page_token: None,
                })
            }
            None => Err(crate::error::Error::NotFound),
        }
    }

    async fn list_share_tables(
        &self,
        request: t::ListShareTablesRequest,
    ) -> Result<t::ListShareTablesResponse> {
        let share_id = Uuid::new_v5(&Uuid::NAMESPACE_OID, request.share.as_bytes());
        match self.shares.get(&request.share) {
            Some(schema_refs) => {
                let tables = schema_refs
                    .iter()
                    .flat_map(|schema_ref| {
                        self.schemas.get(schema_ref).map(|v| {
                            v.iter()
                                .flat_map(|table_ref| {
                                    self.tables.get(table_ref).map(|v| t::Table {
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
                Ok(t::ListShareTablesResponse {
                    items: tables,
                    next_page_token: None,
                })
            }
            None => Err(Error::NotFound),
        }
    }
}

#[async_trait::async_trait]
impl<T: Send + Sync> TableLocationResover for InMemoryHandler<T> {
    async fn resolve(&self, table_ref: &t::TableRef) -> Result<url::Url> {
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
    use crate::policies::DeltaRecipient;

    #[tokio::test]
    async fn test_in_memory_handler() {
        let config = Config {
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

        let shares = handler
            .list_shares(t::ListSharesRequest::default(), DeltaRecipient::Anonymous)
            .await
            .unwrap();
        assert_eq!(shares.items.len(), 1);
        assert_eq!(shares.items[0].name, "share1");

        let share = handler
            .get_share(t::GetShareRequest {
                share: "share1".to_string(),
            })
            .await
            .unwrap();
        assert_eq!(share.share.unwrap().name, "share1");

        let schemas = handler
            .list_schemas(t::ListSchemasRequest {
                share: "share1".to_string(),
                max_results: None,
                page_token: None,
            })
            .await
            .unwrap();
        assert_eq!(schemas.items.len(), 1);
        assert_eq!(schemas.items[0].name, "schema1");

        let tables = handler
            .list_schema_tables(t::ListSchemaTablesRequest {
                share: "share1".to_string(),
                schema: "schema1".to_string(),
                max_results: None,
                page_token: None,
            })
            .await
            .unwrap();
        assert_eq!(tables.items.len(), 1);
        assert_eq!(tables.items[0].name, "table1");

        let tables = handler
            .list_share_tables(t::ListShareTablesRequest {
                share: "share1".to_string(),
                max_results: None,
                page_token: None,
            })
            .await
            .unwrap();
        assert_eq!(tables.items.len(), 1);
        assert_eq!(tables.items[0].name, "table1");
    }
}
