use anyhow::Context;
use anyhow::Result;
use chrono::DateTime;
use chrono::Utc;
use sqlx::postgres::PgQueryResult;
use uuid::Uuid;

use crate::server::entities::schema::Id as SchemaId;
use crate::server::entities::table::Entity;
use crate::server::entities::table::Name;
use crate::server::utilities::postgres::PgAcquire;

#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow)]
pub struct Row {
    pub id: Uuid,
    pub name: String,
    pub schema_id: Uuid,
    pub location: String,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct Repository;

impl Repository {
    pub async fn upsert(table: &Entity, executor: impl PgAcquire<'_>) -> Result<PgQueryResult> {
        let mut conn = executor
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        sqlx::query(
            r#"INSERT INTO "table" (
                   id,
                   name,
                   schema_id,
                   location,
                   created_by
               ) VALUES ($1, $2, $3, $4, $5)
               ON CONFLICT(id)
               DO UPDATE
               SET name = $2,
                   schema_id = $3,
                   location = $4,
                   created_by = $5"#,
        )
        .bind(table.id())
        .bind(table.name())
        .bind(table.schema_id())
        .bind(table.location())
        .bind(table.created_by())
        .execute(&mut *conn)
        .await
        .context(format!(
            r#"failed to upsert "{} ({})" into [table]"#,
            table.name().to_string(),
            table.id().as_uuid()
        ))
    }

    pub async fn select_by_name(
        schema_id: &SchemaId,
        name: &Name,
        executor: impl PgAcquire<'_>,
    ) -> Result<Option<Row>> {
        let mut conn = executor
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        let row: Option<Row> = sqlx::query_as::<_, Row>(
            r#"SELECT
                   id,
                   name,
                   schema_id,
                   location,
                   created_by,
                   created_at,
                   updated_at
               FROM "table"
               WHERE schema_id = $1 AND name = $2"#,
        )
        .bind(schema_id)
        .bind(name)
        .fetch_optional(&mut *conn)
        .await
        .context(format!(
            r#"failed to select "{}" from [table]"#,
            name.as_str()
        ))?;
        Ok(row)
    }
}
