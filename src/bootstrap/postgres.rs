use crate::server::utilities::bootstrap::Utility as BootstrapUtility;
use anyhow::Context;
use anyhow::Result;
use sqlx::PgPool;
use tracing::info;
use tracing::trace;

pub async fn connect(url: &str) -> Result<PgPool> {
    info!("connecting to database");
    let pool = PgPool::connect(&url)
        .await
        .context("failed to acquire postgres connection")?;
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .context("failed to migrate postgres")?;
    trace!("schema created");
    BootstrapUtility::init_postgres(&pool)
        .await
        .context("failed to create admin account")?;
    trace!("admin account created");
    info!("connected to database");
    Ok(pool)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use testcontainers::clients;
    use testcontainers::images::postgres;

    #[derive(sqlx::FromRow)]
    struct Table {
        pub tablename: String,
    }

    #[tokio::test]
    #[ignore]
    async fn test_connect() {
        dotenv::dotenv().ok();
        let docker = clients::Cli::default();
        docker.run(postgres::Postgres::default());
        let url = "postgres://postgres:secret@127.0.0.1:5432";
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
        let pool = connect(&url)
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
    }
}
