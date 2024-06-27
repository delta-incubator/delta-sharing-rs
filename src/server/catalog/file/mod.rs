use serde::Deserialize;
use std::path::Path;

use crate::server::services::{error::Error, schema::Schema, share::Share, table::Table};

use super::{Page, Pagination, ShareStore};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
struct ShareFile {
    shares: Vec<RawShare>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
struct RawShare {
    name: String,
    schemas: Vec<RawSchema>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
struct RawSchema {
    name: String,
    tables: Vec<RawTable>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
struct RawTable {
    name: String,
    location: String,
    id: String,
}

#[derive(Debug)]
pub struct YamlShareStore {
    store: ShareFile,
}

impl YamlShareStore {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(path)?;
        let shares: ShareFile = serde_yaml::from_reader(file)?;
        let yss = YamlShareStore { store: shares };
        Ok(yss)
    }
}

#[async_trait::async_trait]
impl ShareStore for YamlShareStore {
    async fn list_shares(&self, _pagination: &Pagination) -> Result<Page<Share>, Error> {
        let shares = self
            .store
            .shares
            .iter()
            .map(|s| Share {
                id: s.name.clone(),
                name: s.name.clone(),
            })
            .collect::<Vec<_>>();

        Ok(Page {
            items: shares,
            next_page_token: None,
        })
    }

    async fn get_share(&self, name: &str) -> Result<Option<Share>, Error> {
        let share = self
            .store
            .shares
            .iter()
            .find(|s| s.name == name)
            .map(|s| Share {
                id: s.name.clone(),
                name: s.name.clone(),
            });

        Ok(share)
    }

    async fn list_schemas(
        &self,
        share: &str,
        _pagination: &Pagination,
    ) -> Result<Page<Schema>, Error> {
        let share = self
            .store
            .shares
            .iter()
            .find(|s| s.name == share)
            .ok_or_else(|| Error::NotFound)?;

        let schemas = share
            .schemas
            .iter()
            .map(|s| Schema {
                id: s.name.clone(),
                name: s.name.clone(),
            })
            .collect::<Vec<_>>();

        Ok(Page {
            items: schemas,
            next_page_token: None,
        })
    }

    async fn list_tables_in_share(
        &self,
        share: &str,
        _pagination: &Pagination,
    ) -> Result<Page<Table>, Error> {
        let share = self
            .store
            .shares
            .iter()
            .find(|s| s.name == share)
            .ok_or_else(|| Error::NotFound)?;

        let tables = share
            .schemas
            .iter()
            .flat_map(|s| {
                s.tables
                    .iter()
                    .map(|t| Table {
                        id: t.id.clone(),
                        name: t.name.clone(),
                        location: t.location.clone(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Ok(Page {
            items: tables,
            next_page_token: None,
        })
    }

    async fn list_tables_in_schema(
        &self,
        share: &str,
        schema: &str,
        _pagination: &Pagination,
    ) -> Result<Page<Table>, Error> {
        let share = self
            .store
            .shares
            .iter()
            .find(|s| s.name == share)
            .ok_or_else(|| Error::NotFound)?;

        let schema = share
            .schemas
            .iter()
            .find(|s| s.name == schema)
            .ok_or_else(|| Error::NotFound)?;

        let tables = schema
            .tables
            .iter()
            .map(|t| Table {
                id: t.id.clone(),
                name: t.name.clone(),
                location: t.location.clone(),
            })
            .collect::<Vec<_>>();

        Ok(Page {
            items: tables,
            next_page_token: None,
        })
    }

    async fn get_table(&self, share: &str, schema: &str, table: &str) -> Result<Table, Error> {
        let share = self
            .store
            .shares
            .iter()
            .find(|s| s.name == share)
            .ok_or_else(|| Error::NotFound)?;

        let schema = share
            .schemas
            .iter()
            .find(|s| s.name == schema)
            .ok_or_else(|| Error::NotFound)?;

        let table = schema
            .tables
            .iter()
            .find(|t| t.name == table)
            .ok_or_else(|| Error::NotFound)?;

        Ok(Table {
            id: table.id.clone(),
            name: table.name.clone(),
            location: table.location.clone(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_list_shares() {
        let yss =
            YamlShareStore::from_file("./src/server/catalog/file/example-shares.yml").unwrap();
        let shares = yss.list_shares(&Pagination::default()).await.unwrap();

        assert_eq!(shares.items.len(), 4);
        assert_eq!(shares.next_page_token, None);
    }
}
