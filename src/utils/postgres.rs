use crate::config;
use crate::server::entities::account::Entity as Account;
use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use sqlx::postgres::PgDatabaseError;
use sqlx::Acquire;
use sqlx::PgPool;
use sqlx::Postgres;
use tracing::error;
use tracing::info;
use tracing::trace;
use tracing::warn;

const INTEGRITY_ERROR: &str = "23";

pub trait PgAcquire<'c>: Acquire<'c, Database = Postgres> + Send {}

impl<'c, T> PgAcquire<'c> for T where T: Acquire<'c, Database = Postgres> + Send {}

async fn create_admin(pool: &PgPool) -> Result<Account> {
    let admin = if let Ok(admin) = Account::new(
        None,
        config::fetch::<String>("admin_name"),
        config::fetch::<String>("admin_email"),
        config::fetch::<String>("admin_password"),
        config::fetch::<String>("admin_namespace"),
        config::fetch::<i32>("admin_ttl"),
    ) {
        admin
    } else {
        error!("failed to validate admin account");
        return Err(anyhow!("failed to validate admin account"));
    };
    match pg_error(admin.register(&pool).await)? {
        Ok(_) => {
            trace!(
                r#"created admin account id: "{}" name: "{}""#,
                admin.id().as_uuid(),
                admin.name().as_str()
            );
            Ok(admin)
        }
        Err(e) if has_conflict(&e) => {
            warn!("confliction occured while creating admin account: {}", e);
            Ok(admin)
        }
        Err(e) => {
            error!("unknown error occured while creating admin account: {}", e);
            Err(anyhow!("failed to create admin account"))
        }
    }
}

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
    create_admin(&pool)
        .await
        .context("failed to create admin account")?;
    trace!("admin account created");
    info!("connected to database");
    Ok(pool)
}

pub fn pg_error<T>(
    response: anyhow::Result<T>,
) -> Result<std::result::Result<T, Box<PgDatabaseError>>> {
    match response {
        Ok(v) => Ok(Ok(v)),
        Err(e) => match e.downcast::<sqlx::Error>() {
            Ok(sqlx::Error::Database(e)) => Ok(Err(e.downcast::<PgDatabaseError>())),
            _ => Err(anyhow!("unknow database error")),
        },
    }
}

pub fn has_conflict(error: &PgDatabaseError) -> bool {
    &error.code()[..2] == INTEGRITY_ERROR
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
