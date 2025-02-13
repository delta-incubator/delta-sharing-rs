use std::sync::Arc;

use sqlx::migrate::Migrator;
use sqlx::postgres::PgPool;
use url::Url;
use uuid::Uuid;

use crate::error::{Error, Result};

static MIGRATOR: Migrator = sqlx::migrate!();

#[derive(Debug, Clone)]
pub struct TableRecord {
    id: Uuid,
    name: String,
    location: Url,
}

#[async_trait::async_trait]
pub trait SharingRepo {
    async fn add_table(&self, record: &TableRecord) -> Result<Uuid>;
    async fn get_table(&self, id: &Uuid) -> Result<TableRecord>;
    async fn update_table(&self, record: &TableRecord) -> Result<()>;
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
    async fn add_table(&self, record: &TableRecord) -> Result<Uuid> {
        let rec = sqlx::query!(
            r#"
            INSERT INTO table_metadata ( id, name, location )
            VALUES ( $1, $2, $3 )
            RETURNING id
            "#,
            record.id,
            record.name,
            record.location.as_str()
        )
        .fetch_one(&*self.pg_pool)
        .await?;

        Ok(rec.id)
    }

    async fn get_table(&self, id: &Uuid) -> Result<TableRecord> {
        let rec = sqlx::query!(
            "SELECT id, name, location FROM table_metadata WHERE id = $1",
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
        })
    }

    async fn update_table(&self, record: &TableRecord) -> Result<()> {
        let rec = sqlx::query!(
            r#"
            UPDATE table_metadata
            SET name = $1, location = $2
            WHERE id = $3
            "#,
            record.name,
            record.location.as_str(),
            record.id
        )
        .fetch_one(&*self.pg_pool)
        .await?;
        println!("Updated table: {:?}", rec);
        Ok(())
    }
}

pub async fn run_migrations(pool: &PgPool) -> Result<()> {
    Ok(MIGRATOR.run(pool).await?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    async fn test_tables(pool: sqlx::PgPool) -> Result<(), Box<dyn std::error::Error + 'static>> {
        let repo = PgSharingRepo::new(pool);

        let mut record = TableRecord {
            id: Uuid::new_v4(),
            name: "table_name".into(),
            location: Url::parse("file:///location").unwrap(),
        };

        repo.add_table(&record).await?;
        let retrieved = repo.get_table(&record.id).await?;
        assert_eq!(record.id, retrieved.id);

        record.location = Url::parse("file:///location-new").unwrap();
        repo.update_table(&record).await?;

        let retrieved = repo.get_table(&record.id).await?;
        assert_eq!(record.location, retrieved.location);

        Ok(())
    }
}
