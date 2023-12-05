use anyhow::Context;
use anyhow::Result;
use chrono::DateTime;
use chrono::Utc;
use sqlx::postgres::PgQueryResult;
use uuid::Uuid;

use crate::server::entities::account::Entity;
use crate::server::entities::account::Name;
use crate::server::utilities::postgres::PgAcquire;

#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow)]
pub struct Row {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub namespace: String,
    pub ttl: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct Repository;

impl Repository {
    pub async fn upsert(account: &Entity, executor: impl PgAcquire<'_>) -> Result<PgQueryResult> {
        let mut conn = executor
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        sqlx::query(
            "INSERT INTO account (
                 id,
                 name,
                 email,
                 password,
                 namespace,
                 ttl
             ) VALUES ($1, $2, $3, $4, $5, $6)
             ON CONFLICT(id)
             DO UPDATE
             SET name = $2,
                 email = $3,
                 password = $4,
                 namespace = $5,
                 ttl = $6",
        )
        .bind(account.id())
        .bind(account.name())
        .bind(account.email())
        .bind(account.password())
        .bind(account.namespace())
        .bind(account.ttl())
        .execute(&mut *conn)
        .await
        .context(format!(
            r#"failed to upsert "{}" into [account]"#,
            account.id().as_uuid()
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
                 email,
                 password,
                 namespace,
                 ttl,
                 created_at,
                 updated_at
             FROM account
             WHERE name = $1",
        )
        .bind(name)
        .fetch_optional(&mut *conn)
        .await
        .context(format!(
            r#"failed to select "{}" from [account]"#,
            name.as_str()
        ))?;
        Ok(row)
    }
}
