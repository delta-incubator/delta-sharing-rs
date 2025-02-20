use std::sync::Arc;

use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::models::sharing::v1::*;
use crate::{DiscoveryHandler, ResourceRef, TableLocationResover};

pub type DefaultInMemoryHandler = InMemoryHandler;

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
    async fn list_shares(&self, _: ListSharesRequest) -> Result<ListSharesResponse> {
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

    async fn get_share(&self, request: GetShareRequest) -> Result<Share> {
        if self.shares.contains_key(&request.name) {
            let id = Uuid::new_v5(&Uuid::NAMESPACE_OID, request.name.as_bytes());
            Ok(Share {
                id: Some(id.to_string()),
                name: request.name,
            })
        } else {
            Err(Error::NotFound)
        }
    }

    async fn list_schemas(
        &self,
        request: ListSharingSchemasRequest,
    ) -> Result<ListSharingSchemasResponse> {
        match self.shares.get(&request.share) {
            Some(schema_refs) => {
                let schemas = schema_refs
                    .iter()
                    .map(|schema_ref| SharingSchema {
                        name: schema_ref.clone(),
                        share: request.share.clone(),
                        id: None,
                    })
                    .collect();
                Ok(ListSharingSchemasResponse {
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
        if !schema_refs.contains(&request.name) {
            return Err(Error::NotFound);
        }
        let share_id = Uuid::new_v5(&Uuid::NAMESPACE_OID, request.share.as_bytes());
        match self.schemas.get(&request.name) {
            Some(tables) => {
                let tables = tables
                    .iter()
                    .flat_map(|table_ref| {
                        self.tables.get(table_ref).map(|v| SharingTable {
                            id: Some(Uuid::new_v5(&share_id, v.name.as_bytes()).to_string()),
                            name: v.name.clone(),
                            share: request.share.clone(),
                            schema: request.name.clone(),
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
        let share_id = Uuid::new_v5(&Uuid::NAMESPACE_OID, request.name.as_bytes());
        match self.shares.get(&request.name) {
            Some(schema_refs) => {
                let tables = schema_refs
                    .iter()
                    .flat_map(|schema_ref| {
                        self.schemas.get(schema_ref).map(|v| {
                            v.iter()
                                .flat_map(|table_ref| {
                                    self.tables.get(table_ref).map(|v| SharingTable {
                                        id: Some(
                                            Uuid::new_v5(&share_id, v.name.as_bytes()).to_string(),
                                        ),
                                        name: v.name.clone(),
                                        share: request.name.clone(),
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
    async fn resolve(&self, table_ref: &ResourceRef) -> Result<url::Url> {
        match table_ref {
            ResourceRef::Uuid(_) => Err(Error::NotFound),
            ResourceRef::Undefined => Err(Error::NotFound),
            ResourceRef::Name(name) => {
                if name.len() != 3 {
                    return Err(Error::NotFound);
                }
                let Some(schemas) = self.shares.get(&name[0]) else {
                    return Err(Error::NotFound);
                };
                if !schemas.contains(&name[1]) {
                    return Err(Error::NotFound);
                }
                let Some(tables) = self.schemas.get(&name[1]) else {
                    return Err(Error::NotFound);
                };
                if !tables.contains(&name[2]) {
                    return Err(Error::NotFound);
                }
                let table = self.tables.get(&name[2]).ok_or(Error::NotFound)?;
                Ok(url::Url::parse(&table.location)
                    .map_err(|_| Error::InvalidTableLocation(table.location.clone()))?)
            }
        }
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

        let shares = handler
            .list_shares(ListSharesRequest::default())
            .await
            .unwrap();
        assert_eq!(shares.items.len(), 1);
        assert_eq!(shares.items[0].name, "share1");

        let share = handler
            .get_share(GetShareRequest {
                name: "share1".to_string(),
            })
            .await
            .unwrap();
        assert_eq!(share.name, "share1");

        let schemas = handler
            .list_schemas(ListSharingSchemasRequest {
                share: "share1".to_string(),
                max_results: None,
                page_token: None,
            })
            .await
            .unwrap();
        assert_eq!(schemas.items.len(), 1);
        assert_eq!(schemas.items[0].name, "schema1");

        let tables = handler
            .list_schema_tables(ListSchemaTablesRequest {
                share: "share1".to_string(),
                name: "schema1".to_string(),
                max_results: None,
                page_token: None,
            })
            .await
            .unwrap();
        assert_eq!(tables.items.len(), 1);
        assert_eq!(tables.items[0].name, "table1");

        let tables = handler
            .list_share_tables(ListShareTablesRequest {
                name: "share1".to_string(),
                max_results: None,
                page_token: None,
            })
            .await
            .unwrap();
        assert_eq!(tables.items.len(), 1);
        assert_eq!(tables.items[0].name, "table1");
    }
}
