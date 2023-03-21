use crate::server::entities::account::Id as AccountId;
use crate::server::entities::token::Entity;
use crate::server::entities::token::Expiry;
use crate::server::entities::token::Id;
use crate::utils::postgres::PgAcquire;
use anyhow::Context;
use anyhow::Result;
use async_trait::async_trait;
use chrono::DateTime;
use chrono::Utc;
use sqlx::postgres::PgQueryResult;
use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow)]
pub struct Row {
    pub id: Uuid,
    pub expiry: i32,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[async_trait]
pub trait Repository: Send + Sync + 'static {
    async fn upsert(
        &self,
        token: &Entity,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<PgQueryResult>;

    async fn delete(
        &self,
        id: &Id,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<PgQueryResult>;

    async fn select(
        &self,
        limit: Option<&i64>,
        offset: Option<&i64>,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<Vec<Row>>;
}

pub struct PgRepository;

#[async_trait]
impl Repository for PgRepository {
    async fn upsert(
        &self,
        token: &Entity,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<PgQueryResult> {
        let mut conn = executor
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        sqlx::query(
            "INSERT INTO token (
                 id,
                 expiry,
                 created_by
             ) VALUES ($1, $2, $3)
             ON CONFLICT(id)
             DO UPDATE
             SET expiry = $2,
                 created_by = $3",
        )
        .bind(token.id())
        .bind(token.expiry())
        .bind(token.created_by())
        .execute(&mut *conn)
        .await
        .context(format!(
            r#"failed to upsert "{}" into [token]"#,
            token.id().as_uuid()
        ))
    }

    async fn delete(
        &self,
        id: &Id,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<PgQueryResult> {
        let mut conn = executor
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        sqlx::query(
            "DELETE FROM token
             WHERE id = $1",
        )
        .bind(id)
        .execute(&mut *conn)
        .await
        .context(format!(
            r#"failed to delete "{}" from [token]"#,
            id.as_uuid()
        ))
    }

    async fn select(
        &self,
        limit: Option<&i64>,
        offset: Option<&i64>,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<Vec<Row>> {
        let limit = limit.unwrap_or(&10);
        let offset = offset.unwrap_or(&0);
        let mut conn = executor
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        let rows: Vec<Row> = sqlx::query_as::<_, Row>(
            "SELECT
                 id,
                 expiry,
                 created_by,
                 created_at,
                 updated_at
             FROM token
             ORDER BY created_at DESC
             LIMIT $1 OFFSET $2",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&mut *conn)
        .await
        .context(format!("failed to list {} token(s) from [token]", limit))?;
        Ok(rows)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::entities::account::Entity as Account;
    use crate::server::entities::account::Id as AccountId;
    use crate::server::repositories::account::PgRepository as PgAccountRepository;
    use crate::server::repositories::account::Repository as AccountRepository;
    use anyhow::Context;
    use anyhow::Result;
    use sqlx::PgConnection;
    use sqlx::PgPool;
    use std::cmp::min;

    async fn upsert_account(tx: &mut PgConnection) -> Result<Account> {
        let repo = PgAccountRepository;
        let account = Account::new(
            testutils::rand::uuid(),
            testutils::rand::string(10),
            testutils::rand::email(),
            testutils::rand::string(10),
            testutils::rand::string(10),
        )
        .context("failed to upsert account")?;
        repo.upsert(&account, tx)
            .await
            .context("failed to insert account")?;
        Ok(account)
    }

    async fn upsert_token(account_id: &AccountId, tx: &mut PgConnection) -> Result<Entity> {
        let repo = PgRepository;
        let token = Entity::new(
            testutils::rand::uuid(),
            testutils::rand::i32(0, 100000),
            account_id.to_uuid().to_string(),
        )
        .context("failed to upsert token")?;
        repo.upsert(&token, tx)
            .await
            .context("failed to insert token")?;
        Ok(token)
    }

    #[sqlx::test]
    #[ignore] // NOTE: Be sure '$ docker compose -f devops/local/docker-compose.yaml up' before running this test
    async fn test_create_and_select_with_default_limit(pool: PgPool) -> Result<()> {
        let repo = PgRepository;
        let mut tx = pool
            .begin()
            .await
            .expect("transaction should be started properly");
        let account = upsert_account(&mut tx)
            .await
            .expect("new account should be created");
        let records = testutils::rand::i64(0, 20);
        for _ in 0..records {
            upsert_token(account.id(), &mut tx)
                .await
                .expect("new token should be created");
        }
        let fetched = repo
            .select(None, None, &mut tx)
            .await
            .expect("inserted token should be listed");
        assert_eq!(min(records, 10) as usize, fetched.len());
        tx.rollback()
            .await
            .expect("rollback should be done properly");
        Ok(())
    }

    #[sqlx::test]
    #[ignore] // NOTE: Be sure '$ docker compose -f devops/local/docker-compose.yaml up' before running this test
    async fn test_create_and_select_with_specified_limit(pool: PgPool) -> Result<()> {
        let repo = PgRepository;
        let mut tx = pool
            .begin()
            .await
            .expect("transaction should be started properly");
        let account = upsert_account(&mut tx)
            .await
            .expect("new account should be created");
        let records = testutils::rand::i64(0, 20);
        for _ in 0..records {
            upsert_token(account.id(), &mut tx)
                .await
                .expect("new token should be created");
        }
        let limit = testutils::rand::i64(0, 20);
        let fetched = repo
            .select(Some(&limit), None, &mut tx)
            .await
            .expect("inserted token should be listed");
        assert_eq!(min(records, limit) as usize, fetched.len());
        tx.rollback()
            .await
            .expect("rollback should be done properly");
        Ok(())
    }
}
