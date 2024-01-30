use super::services::{error::Error, schema::Schema, share::Share, table::Table};

pub mod file;
pub mod postgres;

/// The `ShareStore` trait defines the interface for fetching metadata about 
/// shares, schemas and tables.
#[async_trait::async_trait]
pub trait ShareStore: Send + Sync {
    /// List all shares.
    async fn list_shares(&self, pagination: &Pagination) -> Result<Page<Share>, Error>;

    /// Get a share by name.
    async fn get_share(&self, name: &str) -> Result<Option<Share>, Error>;

    /// List all schemas in a share.
    async fn list_schemas(
        &self,
        share: &str,
        pagination: &Pagination,
    ) -> Result<Page<Schema>, Error>;

    /// List all tables in a share.
    async fn list_tables_in_share(
        &self,
        share: &str,
        pagination: &Pagination,
    ) -> Result<Page<Table>, Error>;

    /// List all tables in a schema.
    async fn list_tables_in_schema(
        &self,
        share: &str,
        schema: &str,
        pagination: &Pagination,
    ) -> Result<Page<Table>, Error>;

    /// Get a table by name.
    async fn get_table(&self, share: &str, schema: &str, table: &str) -> Result<Table, Error>;
}

/// A set of parameters for paginating a list query.
#[derive(Debug, Clone, Default)]
pub struct Pagination {
    max_results: Option<u32>,
    page_token: Option<String>,
}

impl Pagination {
    pub fn new(max_results: Option<u32>, page_token: Option<String>) -> Self {
        Self {
            max_results,
            page_token,
        }
    }

    pub fn max_results(&self) -> Option<u32> {
        self.max_results
    }

    pub fn page_token(&self) -> Option<&str> {
        self.page_token.as_deref()
    }
}

/// A page of results from a list query.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Page<T> {
    items: Vec<T>,
    next_page_token: Option<String>,
}

impl<T> Page<T> {
    pub fn items(&self) -> &[T] {
        &self.items
    }

    pub fn next_page_token(&self) -> Option<&str> {
        self.next_page_token.as_deref()
    }
}
