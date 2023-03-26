use crate::server::entities::table::Entity as TableEntity;
use crate::server::entities::table::Name as TableName;
use crate::server::utilities::postgres::PgAcquire;
use anyhow::Context;
use anyhow::Result;
use sqlx::query_builder::QueryBuilder;
use sqlx::Execute;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Table {
    pub id: Uuid,
    pub name: String,
    pub location: String,
}

impl Table {
    pub fn from(entity: TableEntity) -> Self {
        Self {
            id: entity.id().to_uuid(),
            name: entity.name().to_string(),
            location: entity.location().to_string(),
        }
    }
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
                   id,
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
        let mut query = sqlx::query_as::<_, Table>(builder.build().sql().into());
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
                   id,
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::entities::account::Entity as AccountEntity;
    use crate::server::entities::account::Id as AccountId;
    use crate::server::entities::table::Entity as TableEntity;
    use crate::server::repositories::account::Repository as AccountRepository;
    use crate::server::repositories::table::Repository as TableRepository;
    use anyhow::Context;
    use anyhow::Result;
    use sqlx::PgConnection;
    use sqlx::PgPool;
    use std::cmp::min;

    async fn create_account(tx: &mut PgConnection) -> Result<AccountEntity> {
        let account = AccountEntity::new(
            testutils::rand::uuid(),
            testutils::rand::string(10),
            testutils::rand::email(),
            testutils::rand::string(10),
            testutils::rand::string(10),
            testutils::rand::i64(1, 100000),
        )
        .context("failed to validate account")?;
        AccountRepository::upsert(&account, tx)
            .await
            .context("failed to create account")?;
        Ok(account)
    }

    async fn create_table(account_id: &AccountId, tx: &mut PgConnection) -> Result<TableEntity> {
        let table = TableEntity::new(
            testutils::rand::uuid(),
            testutils::rand::string(10),
            testutils::rand::string(10),
            account_id.to_uuid().to_string(),
        )
        .context("failed to validate table")?;
        TableRepository::upsert(&table, tx)
            .await
            .context("failed to crate table")?;
        Ok(table)
    }

    #[sqlx::test]
    #[ignore] // NOTE: Be sure '$ docker compose -f devops/local/docker-compose.yaml up' before running this test
    async fn test_create_and_query_with_default_limit(pool: PgPool) -> Result<()> {
        let mut tx = pool
            .begin()
            .await
            .expect("transaction should be started properly");
        let account = create_account(&mut tx)
            .await
            .expect("new account should be created");
        let records = testutils::rand::i64(0, 20);
        for _ in 0..records {
            create_table(account.id(), &mut tx)
                .await
                .expect("new table should be created");
        }
        let fetched = Service::query(None, None, &mut tx)
            .await
            .expect("created table should be listed");
        assert_eq!(records as usize, fetched.len());
        tx.rollback()
            .await
            .expect("rollback should be done properly");
        Ok(())
    }

    #[sqlx::test]
    #[ignore] // NOTE: Be sure '$ docker compose -f devops/local/docker-compose.yaml up' before running this test
    async fn test_create_and_query_with_specified_limit(pool: PgPool) -> Result<()> {
        let mut tx = pool
            .begin()
            .await
            .expect("transaction should be started properly");
        let account = create_account(&mut tx)
            .await
            .expect("new account should be created");
        let records = testutils::rand::i64(0, 20);
        for _ in 0..records {
            create_table(account.id(), &mut tx)
                .await
                .expect("new table should be created");
        }
        let limit = testutils::rand::i64(0, 20);
        let fetched = Service::query(Some(&limit), None, &mut tx)
            .await
            .expect("created table should be listed");
        assert_eq!(min(records, limit) as usize, fetched.len());
        tx.rollback()
            .await
            .expect("rollback should be done properly");
        Ok(())
    }

    #[sqlx::test]
    #[ignore] // NOTE: Be sure '$ docker compose -f devops/local/docker-compose.yaml up' before running this test
    async fn test_create_and_query_by_name(pool: PgPool) -> Result<()> {
        let mut tx = pool
            .begin()
            .await
            .expect("transaction should be started properly");
        let account = create_account(&mut tx)
            .await
            .expect("new account should be created");
        let table = create_table(account.id(), &mut tx)
            .await
            .expect("new table should be created");
        let fetched = Service::query_by_name(&table.name(), &mut tx)
            .await
            .expect("created table should be found");
        if let Some(fetched) = fetched {
            assert_eq!(&fetched.id, table.id().as_uuid());
            assert_eq!(&fetched.name, table.name().as_str());
        } else {
            panic!("created account should be found");
        }
        tx.rollback()
            .await
            .expect("rollback should be done properly");
        Ok(())
    }
}
