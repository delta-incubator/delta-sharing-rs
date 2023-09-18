use anyhow::Context;
use anyhow::Result;
use sqlx::query_builder::QueryBuilder;
use sqlx::Execute;
use utoipa::ToSchema;

use crate::server::entities::share::Entity as ShareEntity;
use crate::server::entities::share::Name as ShareName;
use crate::server::utilities::postgres::PgAcquire;

#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Share {
    pub id: String,
    pub name: String,
}

impl Share {
    pub fn from(entity: ShareEntity) -> Self {
        Self {
            id: entity.id().to_string(),
            name: entity.name().to_string(),
        }
    }
}

pub struct Service;

impl Service {
    pub async fn query(
        limit: Option<&i64>,
        after: Option<&ShareName>,
        executor: impl PgAcquire<'_>,
    ) -> Result<Vec<Share>> {
        let mut conn = executor
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        let mut builder = QueryBuilder::new(
            "SELECT
                 id::text,
                 name
             FROM share",
        );
        if let Some(name) = after {
            builder.push(" WHERE name >= ");
            builder.push_bind(name);
        }
        builder.push(" ORDER BY name ");
        if let Some(limit) = limit {
            builder.push(" LIMIT ");
            builder.push_bind(limit);
        }
        let mut query = sqlx::query_as::<_, Share>(builder.build().sql());
        if let Some(name) = after {
            query = query.bind(name);
        }
        if let Some(limit) = limit {
            query = query.bind(limit);
        }
        let rows: Vec<Share> = query
            .fetch_all(&mut *conn)
            .await
            .context("failed to list shares from [share]")?;
        Ok(rows)
    }

    pub async fn query_by_name(
        name: &ShareName,
        executor: impl PgAcquire<'_>,
    ) -> Result<Option<Share>> {
        let mut conn = executor
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        let row: Option<Share> = sqlx::query_as::<_, Share>(
            "SELECT
                 id::text,
                 name
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
