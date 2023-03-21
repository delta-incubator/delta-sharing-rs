use crate::server::entities::account::Entity;
use crate::server::entities::account::Id;
use crate::server::entities::account::Name;
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
    pub name: String,
    pub email: String,
    pub password: String,
    pub namespace: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[async_trait]
pub trait Repository: Send + Sync + 'static {
    async fn upsert(
        &self,
        account: &Entity,
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

    async fn select_by_id(
        &self,
        id: &Id,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<Option<Row>>;

    async fn select_by_name(
        &self,
        name: &Name,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<Option<Row>>;
}

pub struct PgRepository;

#[async_trait]
impl Repository for PgRepository {
    async fn upsert(
        &self,
        account: &Entity,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<PgQueryResult> {
        let mut conn = executor
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        sqlx::query(
            "INSERT INTO account (
                 id,
                 name,
                 email,
                 password,
                 namespace
             ) VALUES ($1, $2, $3, $4, $5)
             ON CONFLICT(id)
             DO UPDATE
             SET name = $2,
                 email = $3,
                 password = $4,
                 namespace = $5",
        )
        .bind(account.id())
        .bind(account.name())
        .bind(account.email())
        .bind(account.password())
        .bind(account.namespace())
        .execute(&mut *conn)
        .await
        .context(format!(
            r#"failed to upsert "{}" into [account]"#,
            account.id().as_uuid()
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
            "DELETE FROM account
             WHERE id = $1",
        )
        .bind(id)
        .execute(&mut *conn)
        .await
        .context(format!(
            r#"failed to delete "{}" from [account]"#,
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
                 name,
                 email,
                 password,
                 namespace,
                 created_at,
                 updated_at
             FROM account
             ORDER BY created_at DESC
             LIMIT $1 OFFSET $2",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&mut *conn)
        .await
        .context(format!(
            "failed to list {} account(s) from [account]",
            limit
        ))?;
        Ok(rows)
    }

    async fn select_by_id(
        &self,
        id: &Id,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<Option<Row>> {
        let mut conn = executor
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        let row: Option<Row> = sqlx::query_as::<_, Row>(
            "SELECT
                 id,
                 name,
                 email,
                 password,
                 namespace,
                 created_at,
                 updated_at
             FROM account
             WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&mut *conn)
        .await
        .context(format!(
            r#"failed to select "{}" from [account]"#,
            id.as_uuid()
        ))?;
        Ok(row)
    }

    async fn select_by_name(
        &self,
        name: &Name,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<Option<Row>> {
        let mut conn = executor
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        let row: Option<Row> = sqlx::query_as::<_, Row>(
            "SELECT
                 id,
                 name,
                 email,
                 password,
                 namespace,
                 created_at,
                 updated_at
             FROM account
             WHERE name = $1",
        )
        .bind(name)
        .fetch_optional(&mut *conn)
        .await
        .context(format!(
            r#"failed to select "{}" from [account]"#,
            name.as_str()
        ))?;
        Ok(row)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Context;
    use anyhow::Result;
    use sqlx::PgConnection;
    use sqlx::PgPool;
    use std::cmp::min;

    async fn upsert(tx: &mut PgConnection) -> Result<Entity> {
        let repo = PgRepository;
        let account = Entity::new(
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

    #[sqlx::test]
    #[ignore] // NOTE: Be sure '$ docker compose -f devops/local/docker-compose.yaml up' before running this test
    async fn test_create_and_select_with_default_limit(pool: PgPool) -> Result<()> {
        let repo = PgRepository;
        let mut tx = pool
            .begin()
            .await
            .expect("transaction should be started properly");
        let records = testutils::rand::i64(0, 20);
        for _ in 0..records {
            upsert(&mut tx)
                .await
                .expect("new account should be created");
        }
        let fetched = repo
            .select(None, None, &mut tx)
            .await
            .expect("inserted account should be listed");
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
        let records = testutils::rand::i64(0, 20);
        for _ in 0..records {
            upsert(&mut tx)
                .await
                .expect("new account should be created");
        }
        let limit = testutils::rand::i64(0, 20);
        let fetched = repo
            .select(Some(&limit), None, &mut tx)
            .await
            .expect("inserted account should be listed");
        assert_eq!(min(records, limit) as usize, fetched.len());
        tx.rollback()
            .await
            .expect("rollback should be done properly");
        Ok(())
    }

    #[sqlx::test]
    #[ignore] // NOTE: Be sure '$ docker compose -f devops/local/docker-compose.yaml up' before running this test
    async fn test_upsert_and_select_by_id(pool: PgPool) -> Result<()> {
        let repo = PgRepository;
        let mut tx = pool
            .begin()
            .await
            .expect("transaction should be started properly");
        let account = upsert(&mut tx)
            .await
            .expect("new account should be upserted");
        let fetched = repo
            .select_by_id(&account.id(), &mut tx)
            .await
            .expect("inserted account should be found");
        if let Some(fetched) = fetched {
            assert_eq!(&fetched.id, account.id().as_uuid());
            assert_eq!(&fetched.name, account.name().as_str());
            assert_eq!(&fetched.email, account.email().as_str());
            assert_eq!(&fetched.password, account.password().as_str());
            assert_eq!(&fetched.namespace, account.namespace().as_str());
        } else {
            panic!("inserted account should be found");
        }
        tx.rollback()
            .await
            .expect("rollback should be done properly");
        Ok(())
    }

    #[sqlx::test]
    #[ignore] // NOTE: Be sure '$ docker compose -f devops/local/docker-compose.yaml up' before running this test
    async fn test_upsert_and_select_by_name(pool: PgPool) -> Result<()> {
        let repo = PgRepository;
        let mut tx = pool
            .begin()
            .await
            .expect("transaction should be started properly");
        let account = upsert(&mut tx)
            .await
            .expect("new account should be upserted");
        let fetched = repo
            .select_by_name(&account.name(), &mut tx)
            .await
            .expect("inserted account should be found");
        if let Some(fetched) = fetched {
            assert_eq!(&fetched.id, account.id().as_uuid());
            assert_eq!(&fetched.name, account.name().as_str());
            assert_eq!(&fetched.email, account.email().as_str());
            assert_eq!(&fetched.password, account.password().as_str());
            assert_eq!(&fetched.namespace, account.namespace().as_str());
        } else {
            panic!("inserted account should be found");
        }
        tx.rollback()
            .await
            .expect("rollback should be done properly");
        Ok(())
    }
}
