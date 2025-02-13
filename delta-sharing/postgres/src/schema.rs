use std::sync::Arc;

use sqlx::migrate::Migrator;
use sqlx::postgres::PgPool;
use url::Url;
use uuid::Uuid;

use crate::error::{Error, Result};

static MIGRATOR: Migrator = sqlx::migrate!();

#[derive(Debug, Clone)]
pub struct TableRecord {
    pub id: Uuid,
    pub name: String,
    pub location: Url,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[async_trait::async_trait]
pub trait SharingRepo {
    async fn add_table(&self, name: &str, location: &str) -> Result<TableRecord>;
    async fn get_table(&self, id: &Uuid) -> Result<TableRecord>;
    async fn update_table(&self, record: &TableRecord) -> Result<TableRecord>;
}

pub struct PgSharingRepo {
    pg_pool: Arc<PgPool>,
}

impl PgSharingRepo {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pg_pool: Arc::new(pool),
        }
    }

    pub async fn connect(url: impl AsRef<str>) -> Result<Self> {
        let pool = PgPool::connect(url.as_ref()).await?;
        let repo = Self::new(pool);
        repo.test_connection().await?;
        Ok(repo)
    }

    pub async fn migrate(&self) -> Result<()> {
        run_migrations(&self.pg_pool).await
    }

    async fn test_connection(&self) -> Result<()> {
        sqlx::query("SELECT 1").execute(&*self.pg_pool).await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl SharingRepo for PgSharingRepo {
    async fn add_table(&self, name: &str, location: &str) -> Result<TableRecord> {
        let location = Url::parse(location)?;
        let rec = sqlx::query!(
            r#"
            INSERT INTO table_metadata ( name, location )
            VALUES ( $1, $2 )
            RETURNING id, name, location, created_at, updated_at
            "#,
            name,
            location.as_str()
        )
        .fetch_one(&*self.pg_pool)
        .await?;

        Ok(TableRecord {
            id: rec.id,
            name: name.into(),
            location,
            created_at: rec.created_at,
            updated_at: rec.updated_at,
        })
    }

    async fn get_table(&self, id: &Uuid) -> Result<TableRecord> {
        let rec = sqlx::query!(
            r#"
            SELECT id, name, location, created_at, updated_at
            FROM table_metadata
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&*self.pg_pool)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => Error::entity_not_found(id.to_string()),
            err => Error::Connection(err),
        })?;
        Ok(TableRecord {
            id: rec.id,
            name: rec.name,
            location: Url::parse(rec.location.as_str())?,
            created_at: rec.created_at,
            updated_at: rec.updated_at,
        })
    }

    async fn update_table(&self, record: &TableRecord) -> Result<TableRecord> {
        let rec = sqlx::query!(
            r#"
            UPDATE table_metadata
            SET name = $1, location = $2
            WHERE id = $3
            RETURNING id, name, location
            "#,
            record.name,
            record.location.as_str(),
            record.id
        )
        .fetch_one(&*self.pg_pool)
        .await?;
        Ok(TableRecord {
            id: rec.id,
            name: rec.name,
            location: Url::parse(rec.location.as_str())?,
            created_at: record.created_at,
            updated_at: record.updated_at,
        })
    }
}

pub async fn run_migrations(pool: &PgPool) -> Result<()> {
    Ok(MIGRATOR.run(pool).await?)
}
