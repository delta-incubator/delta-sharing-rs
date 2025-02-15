use std::collections::HashMap;

use delta_sharing_common::{Schema, Share};

pub use crate::error::{Error, Result};
pub use graph::*;

mod constants;
mod error;
mod graph;
mod pagination;

#[derive(Debug, Clone)]
pub struct TableRecord {
    pub id: uuid::Uuid,
    pub name: String,
    pub location: url::Url,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub enum ShareRef {
    Uuid(uuid::Uuid),
    Name(String),
}

pub enum SchemaRef {
    Uuid(uuid::Uuid),
    Name((Vec<String>, String)),
}

#[async_trait::async_trait]
pub trait SharingRepo {
    async fn add_share(
        &self,
        name: &str,
        comment: Option<String>,
        properties: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<Share>;
    async fn get_share(&self, id: &ShareRef) -> Result<Share>;
    async fn delete_share(&self, id: &ShareRef) -> Result<()>;

    /// List shares.
    async fn list_shares(
        &self,
        max_results: Option<usize>,
        page_token: Option<&str>,
    ) -> Result<(Vec<Share>, Option<String>)>;

    /// Add a schema.
    async fn add_schema(
        &self,
        share: &str,
        name: &str,
        comment: Option<String>,
        properties: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<Schema>;

    /// Get a schema.
    async fn get_schema(&self, id: &SchemaRef) -> Result<Schema>;

    /// Delete a schema.
    async fn delete_schema(&self, id: &SchemaRef) -> Result<()>;

    /// List schemas.
    async fn list_schemas(
        &self,
        share: &str,
        max_results: Option<usize>,
        page_token: Option<&str>,
    ) -> Result<(Vec<Schema>, Option<String>)>;

    // async fn add_table(&self, name: &str, location: &str) -> Result<TableRecord>;
    // async fn get_table(&self, id: &uuid::Uuid) -> Result<TableRecord>;
    // async fn update_table(&self, record: &TableRecord) -> Result<TableRecord>;
}
