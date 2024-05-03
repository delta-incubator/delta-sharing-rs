use std::path::Path;
use std::sync::Arc;

use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::traits::DiscoveryHandler;
use crate::types as t;

#[derive(Debug, Serialize, Deserialize)]
struct TableConfig {
    name: String,
    location: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SchemaConfig {
    name: String,
    table_refs: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ShareConfig {
    name: String,
    schema_refs: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    shares: Vec<ShareConfig>,
    schemas: Vec<SchemaConfig>,
    tables: Vec<TableConfig>,
}

impl Config {
    pub fn read(path: impl AsRef<Path>) -> Result<Self> {
        let config = std::fs::read_to_string(path)?;
        Ok(serde_yml::from_str(&config)?)
    }
}

pub struct InMemoryHandler {
    // The data in memory
    shares: Arc<DashMap<String, Vec<String>>>,
    schemas: Arc<DashMap<String, Vec<String>>>,
    tables: Arc<DashMap<String, TableConfig>>,
}

impl InMemoryHandler {
    pub fn try_new(config_path: impl AsRef<Path>) -> Result<Self> {
        let config = Config::read(config_path)?;
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

        Ok(Self {
            shares,
            schemas,
            tables,
        })
    }
}

#[async_trait::async_trait]
impl DiscoveryHandler for InMemoryHandler {
    type Recipient = ();

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

    async fn get_share(
        &self,
        request: t::GetShareRequest,
        _recipient: Self::Recipient,
    ) -> Result<t::GetShareResponse> {
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

    async fn list_schemas(
        &self,
        request: t::ListSchemasRequest,
        _recipient: Self::Recipient,
    ) -> Result<t::ListSchemasResponse> {
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
        _recipient: Self::Recipient,
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
        _recipient: Self::Recipient,
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
