use crate::error::Result;
pub use crate::schema::PgSharingRepo;

mod error;
mod schema;

#[derive(Debug, Clone)]
pub struct TableRecord {
    pub id: uuid::Uuid,
    pub name: String,
    pub location: url::Url,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[async_trait::async_trait]
pub trait SharingRepo {
    async fn add_table(&self, name: &str, location: &str) -> Result<TableRecord>;
    async fn get_table(&self, id: &uuid::Uuid) -> Result<TableRecord>;
    async fn update_table(&self, record: &TableRecord) -> Result<TableRecord>;
}
