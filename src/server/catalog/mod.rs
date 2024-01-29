use super::services::{error::Error, schema::Schema, share::Share, table::Table};

mod file;
mod postgres;

#[async_trait::async_trait]
pub trait ShareStore: Send + Sync {
    async fn list_shares(&self, pagination: &Pagination) -> Result<Page<Share>, Error>;

    async fn get_share(&self, name: &str) -> Result<Option<Share>, Error>;

    async fn list_schemas(
        &self,
        share: &str,
        pagination: &Pagination,
    ) -> Result<Page<Schema>, Error>;

    async fn list_tables_in_share(
        &self,
        share: &str,
        pagination: &Pagination,
    ) -> Result<Page<Table>, Error>;

    async fn list_tables_in_schema(
        &self,
        share: &str,
        schema: &str,
        pagination: &Pagination,
    ) -> Result<Page<Table>, Error>;

    async fn get_table(&self, share: &str, schema: &str, table: &str) -> Result<Table, Error>;
}

#[derive(Debug, Clone, Default)]
pub struct Pagination {
    max_results: Option<u32>,
    page_token: Option<String>,
}

impl Pagination {
    pub fn max_results(&self) -> Option<u32> {
        self.max_results
    }

    pub fn page_token(&self) -> Option<&str> {
        self.page_token.as_deref()
    }
}

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
