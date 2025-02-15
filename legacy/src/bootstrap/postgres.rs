use anyhow::{Context, Result};
use sqlx::PgPool;

use crate::server::utilities::bootstrap::Utility as BootstrapUtility;

pub async fn connect(url: &str) -> Result<PgPool> {
    tracing::info!("connecting to database");
    let pool = PgPool::connect(url)
        .await
        .context("failed to acquire postgres connection")?;
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .context("failed to migrate postgres")?;
    tracing::trace!("migrated tables");
    BootstrapUtility::init_postgres(&pool)
        .await
        .context("failed to create admin account")?;
    tracing::trace!("bootstrapped database");
    tracing::info!("connected to database");
    Ok(pool)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use testcontainers::runners::AsyncRunner;
    use testcontainers_modules::postgres::Postgres;

    #[derive(sqlx::FromRow)]
    struct Table {
        pub tablename: String,
    }

    #[tokio::test]
    #[ignore]
    async fn test_connect() -> Result<(), Box<dyn std::error::Error + 'static>> {
        dotenv::dotenv().ok();

        let node = Postgres::default().with_password("secret").start().await?;

        let connection_string = &format!(
            "postgres://postgres:secret@127.0.0.1:{}/postgres",
            node.get_host_port_ipv4(5432).await?
        );

        let expected: HashSet<_> = [
            String::from("account"),
            String::from("share"),
            String::from("table"),
            String::from("schema"),
            String::from("token"),
        ]
        .iter()
        .cloned()
        .collect();
        let pool = connect(connection_string)
            .await
            .expect("connection should be established");
        let tables: HashSet<String> = HashSet::from_iter(
            sqlx::query_as::<_, Table>(
                "SELECT *
                 FROM pg_catalog.pg_tables
                 WHERE schemaname != 'pg_catalog' AND
                       schemaname != 'information_schema' AND
                       tablename != '_sqlx_migrations'",
            )
            .fetch_all(&pool)
            .await
            .expect("table names should be queried")
            .into_iter()
            .map(|t: Table| t.tablename)
            .collect::<Vec<String>>(),
        );
        assert_eq!(&expected, &tables);

        Ok(())
    }
}
