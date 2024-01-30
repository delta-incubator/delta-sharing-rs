use anyhow::{Context, Result};
use sqlx::query_builder::QueryBuilder;
use sqlx::Execute;
use utoipa::ToSchema;

use crate::server::entities::schema::Name as SchemaName;
use crate::server::entities::share::Name as ShareName;
use crate::server::entities::table::Entity as TableEntity;
use crate::server::entities::table::Name as TableName;
use crate::server::utilities::postgres::PgAcquire;

#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Table {
    pub id: String,
    pub name: String,
    pub location: String,
}

impl Table {
    pub fn from(entity: TableEntity) -> Self {
        Self {
            id: entity.id().to_string(),
            name: entity.name().to_string(),
            location: entity.location().to_string(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TableDetail {
    pub name: String,
    pub schema: String,
    pub share: String,
}

pub struct Service;

impl Service {
    pub async fn query(
        limit: Option<&i64>,
        after: Option<&TableName>,
        executor: impl PgAcquire<'_>,
    ) -> Result<Vec<Table>> {
        let mut conn = executor
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        let mut builder = QueryBuilder::new(
            r#"SELECT
                   id::text,
                   name,
                   location
               FROM "table""#,
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
        let mut query = sqlx::query_as::<_, Table>(builder.build().sql());
        if let Some(name) = after {
            query = query.bind(name);
        }
        if let Some(limit) = limit {
            query = query.bind(limit);
        }
        let rows: Vec<Table> = query
            .fetch_all(&mut *conn)
            .await
            .context("failed to list tables from [table]")?;
        Ok(rows)
    }

    pub async fn query_by_name(
        name: &TableName,
        executor: impl PgAcquire<'_>,
    ) -> Result<Option<Table>> {
        let mut conn = executor
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        let row: Option<Table> = sqlx::query_as::<_, Table>(
            r#"SELECT
                   id::text,
                   name,
                   location
               FROM "table"
               WHERE name = $1"#,
        )
        .bind(name)
        .fetch_optional(&mut *conn)
        .await
        .context(format!(
            r#"failed to select "{}" from [table]"#,
            name.as_str()
        ))?;
        Ok(row)
    }

    pub async fn query_by_fqn(
        share_name: &ShareName,
        schema_name: &SchemaName,
        table_name: &TableName,
        executor: impl PgAcquire<'_>,
    ) -> Result<Option<Table>> {
        let mut conn = executor
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        let row: Option<Table> = sqlx::query_as::<_, Table>(
            r#"SELECT
                   "table".id::text AS id,
                   "table".name AS name,
                   "table".location AS location
               FROM "table"
               LEFT JOIN "schema" ON "schema".id = "table".schema_id
               LEFT JOIN share ON share.id = "schema".share_id
               WHERE share.name = $1 AND "schema".name = $2 AND "table".name = $3"#,
        )
        .bind(share_name)
        .bind(schema_name)
        .bind(table_name)
        .fetch_optional(&mut *conn)
        .await
        .context(format!(
            r#"failed to select "{}"/"{}"/"{}" from [table]"#,
            share_name.as_str(),
            schema_name.as_str(),
            table_name.as_str(),
        ))?;
        Ok(row)
    }

    pub async fn query_by_share_name(
        share_name: &ShareName,
        limit: Option<&i64>,
        after: Option<&TableName>,
        executor: impl PgAcquire<'_>,
    ) -> Result<Vec<TableDetail>> {
        let mut conn = executor
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        let mut builder = QueryBuilder::new(
            r#"WITH these_tables AS (
                   SELECT
                       "table".id AS id,
                       share.id AS share_id,
                       "table".name AS name,
                       "schema".name AS schema,
                       share.name AS share
                   FROM "table"
                   LEFT JOIN "schema" ON "schema".id = "table".schema_id
                   LEFT JOIN share ON share.id = "schema".share_id
                   WHERE share.name = "#,
        );
        builder.push_bind(share_name);
        builder.push(
            "
               )
               SELECT
                   name,
                   schema,
                   share
               FROM these_tables",
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
        let mut query = sqlx::query_as::<_, TableDetail>(builder.build().sql());
        query = query.bind(share_name);
        if let Some(name) = after {
            query = query.bind(name);
        }
        if let Some(limit) = limit {
            query = query.bind(limit);
        }
        let rows: Vec<TableDetail> = query
            .fetch_all(&mut *conn)
            .await
            .context("failed to list tables from [table]")?;
        Ok(rows)
    }

    pub async fn query_by_share_and_schema_name(
        share_name: &ShareName,
        schema_name: &SchemaName,
        limit: Option<&i64>,
        after: Option<&TableName>,
        executor: impl PgAcquire<'_>,
    ) -> Result<Vec<TableDetail>> {
        let mut conn = executor
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        let mut builder = QueryBuilder::new(
            r#"WITH these_tables AS (
                   SELECT
                       "table".id AS id,
                       share.id AS share_id,
                       "table".name AS name,
                       "schema".name AS schema,
                       share.name AS share
                   FROM "table"
                   LEFT JOIN "schema" ON "schema".id = "table".schema_id
                   LEFT JOIN share ON share.id = "schema".share_id
                   WHERE share.name = "#,
        );
        builder.push_bind(share_name);
        builder.push(r#" AND "schema".name = "#);
        builder.push_bind(schema_name);
        builder.push(
            "
               )
               SELECT
                   id::text,
                   share_id::text,
                   name,
                   schema,
                   share
               FROM these_tables",
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
        let mut query = sqlx::query_as::<_, TableDetail>(builder.build().sql());
        query = query.bind(share_name);
        query = query.bind(schema_name);
        if let Some(name) = after {
            query = query.bind(name);
        }
        if let Some(limit) = limit {
            query = query.bind(limit);
        }
        let rows: Vec<TableDetail> = query
            .fetch_all(&mut *conn)
            .await
            .context("failed to list tables from [table]")?;
        Ok(rows)
    }
}
