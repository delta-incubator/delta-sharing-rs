use anyhow::Context;
use anyhow::Result;
use sqlx::query_builder::QueryBuilder;
use sqlx::Execute;
use utoipa::ToSchema;

use crate::server::entities::schema::Entity as SchemaEntity;
use crate::server::entities::schema::Name as SchemaName;
use crate::server::entities::share::Name as ShareName;
use crate::server::utilities::postgres::PgAcquire;

#[derive(Debug, Clone, serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    pub id: String,
    pub name: String,
}

impl Schema {
    pub fn from(entity: SchemaEntity) -> Self {
        Self {
            id: entity.id().to_string(),
            name: entity.name().to_string(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SchemaDetail {
    pub name: String,
    pub share: String,
}

pub struct Service;

impl Service {
    pub async fn query_by_share_name(
        share_name: &ShareName,
        limit: Option<&i64>,
        after: Option<&SchemaName>,
        executor: impl PgAcquire<'_>,
    ) -> Result<Vec<SchemaDetail>> {
        let mut conn = executor
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        let mut builder = QueryBuilder::new(
            r#"WITH these_schemas AS (
                   SELECT
                       DISTINCT "schema".name AS name, share.name AS share
                   FROM "schema"
                   LEFT JOIN share ON share.id = "schema".share_id
                   WHERE share.name = "#,
        );
        builder.push_bind(share_name);
        builder.push(
            "
               )
               SELECT
                   name,
                   share
               FROM these_schemas",
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
        let mut query = sqlx::query_as::<_, SchemaDetail>(builder.build().sql());
        query = query.bind(share_name);
        if let Some(name) = after {
            query = query.bind(name);
        }
        if let Some(limit) = limit {
            query = query.bind(limit);
        }
        let rows: Vec<SchemaDetail> = query
            .fetch_all(&mut *conn)
            .await
            .context("failed to list schemas from [schema]")?;
        Ok(rows)
    }
}
