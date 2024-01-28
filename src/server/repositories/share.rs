use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use sqlx::postgres::PgQueryResult;
use uuid::Uuid;

use crate::server::entities::share::{Entity, Name};
use crate::server::utilities::postgres::PgAcquire;

#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow)]
pub struct Row {
    pub id: Uuid,
    pub name: String,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct Repository;

impl Repository {
    pub async fn upsert(share: &Entity, executor: impl PgAcquire<'_>) -> Result<PgQueryResult> {
        let mut conn = executor
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        sqlx::query(
            "INSERT INTO share (
                 id,
                 name,
                 created_by
             ) VALUES ($1, $2, $3)
             ON CONFLICT(id)
             DO UPDATE
             SET name = $2,
                 created_by = $3",
        )
        .bind(share.id())
        .bind(share.name())
        .bind(share.created_by())
        .execute(&mut *conn)
        .await
        .context(format!(
            r#"failed to upsert "{}" into [share]"#,
            share.id().as_uuid()
        ))
    }

    pub async fn select_by_name(name: &Name, executor: impl PgAcquire<'_>) -> Result<Option<Row>> {
        let mut conn = executor
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        let row: Option<Row> = sqlx::query_as::<_, Row>(
            "SELECT
                 id,
                 name,
                 created_by,
                 created_at,
                 updated_at
             FROM share
             WHERE name = $1",
        )
        .bind(name)
        .fetch_optional(&mut *conn)
        .await
        .context(format!(
            r#"failed to select "{}" from [share]"#,
            name.as_str()
        ))?;
        Ok(row)
    }
}
