use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use sqlx::postgres::PgQueryResult;
use uuid::Uuid;

use crate::server::entities::token::Entity;
use crate::server::middlewares::jwt::Role;
use crate::server::utilities::postgres::PgAcquire;

#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow)]
pub struct Row {
    pub id: Uuid,
    pub email: String,
    pub role: Role,
    pub value: String,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct Repository;

impl Repository {
    pub async fn upsert(token: &Entity, executor: impl PgAcquire<'_>) -> Result<PgQueryResult> {
        let mut conn = executor
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        sqlx::query(
            r#"INSERT INTO token (
                   id,
                   email,
                   "role",
                   "value",
                   created_by
               ) VALUES ($1, $2, $3, $4, $5)
               ON CONFLICT(id)
               DO UPDATE
               SET email = $2,
                   "role" = $3,
                   "value" = $4,
                   created_by = $5"#,
        )
        .bind(token.id())
        .bind(token.email())
        .bind(token.role())
        .bind(token.value())
        .bind(token.created_by())
        .execute(&mut *conn)
        .await
        .context(format!(
            r#"failed to upsert "{}" into [token]"#,
            token.id().as_uuid()
        ))
    }
}
