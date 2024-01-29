use anyhow::{Context, Result};
use sqlx::{Execute, PgPool, Postgres, QueryBuilder};

use crate::server::services::{error::Error, schema::Schema, share::Share, table::Table};

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
        unimplemented!()
    }

    async fn list_schemas(
        &self,
        share: &str,
        pagination: &Pagination,
    ) -> Result<Page<Schema>, Error> {
        unimplemented!()
    }

    async fn list_tables_in_share(
        &self,
        share: &str,
        pagination: &Pagination,
    ) -> Result<Page<Table>, Error> {
        unimplemented!()
    }

    async fn list_tables_in_schema(
        &self,
        share: &str,
        schema: &str,
        pagination: &Pagination,
    ) -> Result<Page<Table>, Error> {
        unimplemented!()
    }

    async fn get_table(&self, share: &str, schema: &str, table: &str) -> Result<Table, Error> {
        unimplemented!()
    }
}
