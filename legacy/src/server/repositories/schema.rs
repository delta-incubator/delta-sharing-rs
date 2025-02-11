use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use sqlx::postgres::PgQueryResult;
use uuid::Uuid;

use crate::server::entities::schema::{Entity, Name};
use crate::server::entities::share::Id as ShareId;
use crate::server::utilities::postgres::PgAcquire;

#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow)]
pub struct Row {
    pub id: Uuid,
    pub name: String,
    pub share_id: Uuid,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct Repository;

impl Repository {
    pub async fn upsert(schema: &Entity, executor: impl PgAcquire<'_>) -> Result<PgQueryResult> {
        let mut conn = executor
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        sqlx::query(
            r#"INSERT INTO "schema" (
                   id,
                   name,
                   share_id,
                   created_by
               ) VALUES ($1, $2, $3, $4)
               ON CONFLICT(id)
               DO UPDATE
               SET name = $2,
                   share_id = $3,
                   created_by = $4"#,
        )
        .bind(schema.id())
        .bind(schema.name())
        .bind(schema.share_id())
        .bind(schema.created_by())
        .execute(&mut *conn)
        .await
        .context(format!(
            r#"failed to upsert "{}" into [schema]"#,
            schema.id().as_uuid()
        ))
    }

    pub async fn select_by_name(
        share_id: &ShareId,
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
                 share_id,
                 created_by,
                 created_at,
                 updated_at
             FROM "schema"
             WHERE share_id = $1 AND name = $2"#,
        )
        .bind(share_id)
        .bind(name)
        .fetch_optional(&mut *conn)
        .await
        .context(format!(
            r#"failed to select "{}" from [schema]"#,
            name.as_str()
        ))?;
        Ok(row)
    }
}
