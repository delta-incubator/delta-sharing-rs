use anyhow::Context;
use anyhow::Result;
use chrono::DateTime;
use chrono::Utc;
use sqlx::postgres::PgQueryResult;
use uuid::Uuid;

use crate::server::entities::schema::Entity;
use crate::server::entities::schema::Name;
use crate::server::entities::share::Id as ShareId;

use crate::server::utilities::postgres::PgAcquire;

#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow)]
pub struct Row {
    pub id: Uuid,
    pub name: String,
    pub share_id: Uuid,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct Repository;

impl Repository {
    pub async fn upsert(schema: &Entity, executor: impl PgAcquire<'_>) -> Result<PgQueryResult> {
        let mut conn = executor
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        sqlx::query(
            r#"INSERT INTO "schema" (
                   id,
                   name,
                   share_id,
                   created_by
               ) VALUES ($1, $2, $3, $4)
               ON CONFLICT(id)
               DO UPDATE
               SET name = $2,
                   share_id = $3,
                   created_by = $4"#,
        )
        .bind(schema.id())
        .bind(schema.name())
        .bind(schema.share_id())
        .bind(schema.created_by())
        .execute(&mut *conn)
        .await
        .context(format!(
            r#"failed to upsert "{}" into [schema]"#,
            schema.id().as_uuid()
        ))
    }

    pub async fn select_by_name(
        share_id: &ShareId,
        name: &Name,
        executor: impl PgAcquire<'_>,
    ) -> Result<Option<Row>> {
        let mut conn = executor
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        let row: Option<Row> = sqlx::query_as::<_, Row>(
            r#"SELECT
                 id,
                 name,
                 share_id,
                 created_by,
                 created_at,
                 updated_at
             FROM "schema"
             WHERE share_id = $1 AND name = $2"#,
        )
        .bind(share_id)
        .bind(name)
        .fetch_optional(&mut *conn)
        .await
        .context(format!(
            r#"failed to select "{}" from [schema]"#,
            name.as_str()
        ))?;
        Ok(row)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Context;
    use anyhow::Result;
    use sqlx::PgConnection;
    use sqlx::PgPool;

    use super::*;
    use crate::server::entities::account::Entity as Account;
    use crate::server::entities::account::Id as AccountId;
    use crate::server::entities::share::Entity as Share;
    use crate::server::repositories::account::Repository as AccountRepository;
    use crate::server::repositories::share::Repository as ShareRepository;

    async fn create_account(tx: &mut PgConnection) -> Result<Account> {
        let account = Account::new(
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

    async fn create_share(account_id: &AccountId, tx: &mut PgConnection) -> Result<Share> {
        let share = Share::new(
            testutils::rand::uuid(),
            testutils::rand::string(10),
            account_id.to_uuid().to_string(),
        )
        .context("failed to validate share")?;
        ShareRepository::upsert(&share, tx)
            .await
            .context("failed to create share")?;
        Ok(share)
    }

    async fn create_schema(
        account_id: &AccountId,
        share_id: &ShareId,
        tx: &mut PgConnection,
    ) -> Result<Entity> {
        let schema = Entity::new(
            testutils::rand::uuid(),
            testutils::rand::string(10),
            share_id.to_uuid().to_string(),
            account_id.to_uuid().to_string(),
        )
        .context("failed to validate schema")?;
        Repository::upsert(&schema, tx)
            .await
            .context("failed to create schema")?;
        Ok(schema)
    }

    #[sqlx::test]
    #[ignore] // NOTE: Be sure '$ docker compose -f devops/local/docker-compose.yaml up' before running this test
    async fn test_create(pool: PgPool) -> Result<()> {
        let mut tx = pool
            .begin()
            .await
            .expect("transaction should be started properly");
        let account = create_account(&mut tx)
            .await
            .expect("new account should be created");
        let share = create_share(account.id(), &mut tx)
            .await
            .expect("new share should be created");
        let records = testutils::rand::i64(0, 20);
        for _ in 0..records {
            create_schema(account.id(), share.id(), &mut tx)
                .await
                .expect("new schema should be created");
        }
        tx.rollback()
            .await
            .expect("rollback should be done properly");
        Ok(())
    }
}
