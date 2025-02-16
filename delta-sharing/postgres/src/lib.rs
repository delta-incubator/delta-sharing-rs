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
