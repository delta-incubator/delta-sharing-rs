use anyhow::{Context, Result};
use sqlx::{Execute, PgPool, Postgres, QueryBuilder};

use crate::server::services::{
    error::Error,
    schema::{Schema, SchemaDetail},
    share::Share,
    table::{Table, TableDetail},
};

use super::{Page, Pagination, ShareStore};

pub struct PostgresShareStore {
    pool: PgPool,
}

impl PostgresShareStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn query(&self, limit: Option<u32>, after: Option<&str>) -> Result<Vec<Share>> {
        let mut conn = self
            .pool
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        let mut builder: QueryBuilder<'_, Postgres> = QueryBuilder::new(
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
            builder.push_bind(limit as i32);
        }
        let mut query = sqlx::query_as::<_, Share>(builder.build().sql());
        if let Some(name) = after {
            query = query.bind(name);
        }
        if let Some(limit) = limit {
            query = query.bind(limit as i32);
        }
        let rows: Vec<Share> = query
            .fetch_all(&mut *conn)
            .await
            .context("failed to list shares from [share]")?;
        Ok(rows)
    }

    pub async fn query_by_name(&self, name: &str) -> Result<Option<Share>> {
        let mut conn = self
            .pool
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
        .context(format!(r#"failed to select "{}" from [share]"#, name))?;
        Ok(row)
    }

    pub async fn query_by_share_name(
        &self,
        share_name: &str,
        limit: Option<u32>,
        after: Option<&str>,
    ) -> Result<Vec<SchemaDetail>> {
        let mut conn = self
            .pool
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        let mut builder: QueryBuilder<'_, Postgres> = QueryBuilder::new(
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
            builder.push_bind(limit as i64);
        }
        let mut query = sqlx::query_as::<_, SchemaDetail>(builder.build().sql());
        query = query.bind(share_name);
        if let Some(name) = after {
            query = query.bind(name);
        }
        if let Some(limit) = limit {
            query = query.bind(limit as i64);
        }
        let rows: Vec<SchemaDetail> = query
            .fetch_all(&mut *conn)
            .await
            .context("failed to list schemas from [schema]")?;
        Ok(rows)
    }

    pub async fn query_table_by_share_name(
        &self,
        share_name: &str,
        limit: Option<u32>,
        after: Option<&str>,
    ) -> Result<Vec<TableDetail>> {
        let mut conn = self
            .pool
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        let mut builder: QueryBuilder<'_, Postgres> = QueryBuilder::new(
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
            builder.push_bind(limit as i64);
        }
        let mut query = sqlx::query_as::<_, TableDetail>(builder.build().sql());
        query = query.bind(share_name);
        if let Some(name) = after {
            query = query.bind(name);
        }
        if let Some(limit) = limit {
            query = query.bind(limit as i64);
        }
        let rows: Vec<TableDetail> = query
            .fetch_all(&mut *conn)
            .await
            .context("failed to list tables from [table]")?;
        Ok(rows)
    }

    pub async fn query_by_fqn(
        &self,
        share_name: &str,
        schema_name: &str,
        table_name: &str,
    ) -> Result<Option<Table>> {
        let mut conn = self
            .pool
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
            share_name, schema_name, table_name,
        ))?;
        Ok(row)
    }

    pub async fn query_by_share_and_schema_name(
        &self,
        share_name: &str,
        schema_name: &str,
        limit: Option<u32>,
        after: Option<&str>,
    ) -> Result<Vec<TableDetail>> {
        let mut conn = self
            .pool
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        let mut builder: QueryBuilder<'_, Postgres> = QueryBuilder::new(
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
            builder.push_bind(limit as i64);
        }
        let mut query = sqlx::query_as::<_, TableDetail>(builder.build().sql());
        query = query.bind(share_name);
        query = query.bind(schema_name);
        if let Some(name) = after {
            query = query.bind(name);
        }
        if let Some(limit) = limit {
            query = query.bind(limit as i64);
        }
        let rows: Vec<TableDetail> = query
            .fetch_all(&mut *conn)
            .await
            .context("failed to list tables from [table]")?;
        Ok(rows)
    }
}

#[async_trait::async_trait]
impl ShareStore for PostgresShareStore {
    async fn list_shares(&self, pagination: &Pagination) -> Result<Page<Share>, Error> {
        let limit = pagination.max_results().unwrap_or(500);
        let after = pagination.page_token();

        let shares = self.query(Some(limit), after).await?;
        let next = if shares.len() < limit as usize {
            None
        } else {
            shares.last().map(|s| s.name.clone())
        };

        Ok(Page {
            items: shares,
            next_page_token: next,
        })
    }

    async fn get_share(&self, name: &str) -> Result<Option<Share>, Error> {
        let share = self.query_by_name(name).await?;
        Ok(share)
    }

    async fn list_schemas(
        &self,
        share: &str,
        pagination: &Pagination,
    ) -> Result<Page<Schema>, Error> {
        let limit = pagination.max_results().unwrap_or(500);
        let after = pagination.page_token();

        let schemas = self.query_by_share_name(share, Some(limit), after).await?;
        let next = if schemas.len() < limit as usize {
            None
        } else {
            schemas.last().map(|s| s.name.clone())
        };

        Ok(Page {
            items: schemas
                .into_iter()
                .map(|s| Schema {
                    name: s.name,
                    id: String::new(),
                })
                .collect(),
            next_page_token: next,
        })
    }

    async fn list_tables_in_share(
        &self,
        share: &str,
        pagination: &Pagination,
    ) -> Result<Page<Table>, Error> {
        let limit = pagination.max_results().unwrap_or(500);
        let after = pagination.page_token();

        let tables = self
            .query_table_by_share_name(share, Some(limit), after)
            .await?;
        let next = if tables.len() < limit as usize {
            None
        } else {
            tables.last().map(|s| s.name.clone())
        };

        Ok(Page {
            items: tables
                .into_iter()
                .map(|t: TableDetail| Table {
                    id: String::new(),
                    name: t.name,
                    location: String::new(),
                })
                .collect(),
            next_page_token: next,
        })
    }

    async fn list_tables_in_schema(
        &self,
        share: &str,
        schema: &str,
        pagination: &Pagination,
    ) -> Result<Page<Table>, Error> {
        let limit = pagination.max_results().unwrap_or(500);
        let after = pagination.page_token();

        let tables = self
            .query_by_share_and_schema_name(share, schema, Some(limit), after)
            .await?;
        let next = if tables.len() < limit as usize {
            None
        } else {
            tables.last().map(|s| s.name.clone())
        };

        Ok(Page {
            items: tables
                .into_iter()
                .map(|t: TableDetail| Table {
                    id: String::new(),
                    name: t.name,
                    location: String::new(),
                })
                .collect(),
            next_page_token: next,
        })
    }

    async fn get_table(&self, share: &str, schema: &str, table: &str) -> Result<Table, Error> {
        let table = self.query_by_fqn(share, schema, table).await?;

        if let Some(t) = table {
            Ok(t)
        } else {
            Err(Error::NotFound)
        }
    }
}
