use crate::server::entities::share::Entity;
use crate::server::entities::share::Id;
use crate::server::entities::share::Name;
use crate::server::utils::postgres::PgAcquire;
use anyhow::Context;
use anyhow::Result;
use async_trait::async_trait;
use chrono::DateTime;
use chrono::Utc;
use sqlx::postgres::PgQueryResult;
use sqlx::query_builder::QueryBuilder;
use sqlx::Execute;
use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow)]
pub struct Row {
    pub id: Uuid,
    pub name: String,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[async_trait]
pub trait Repository: Send + Sync + 'static {
    async fn upsert(
        &self,
        share: &Entity,
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
        after: Option<&Name>,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<Vec<Row>>;

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
        share: &Entity,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<PgQueryResult> {
        let mut conn = executor
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        sqlx::query(
            "INSERT INTO share (
                 id,
                 name,
                 created_by
             ) VALUES ($1, $2, $3)
             ON CONFLICT(id)
             DO UPDATE
             SET name = $2,
                 created_by = $3",
        )
        .bind(share.id())
        .bind(share.name())
        .bind(share.created_by())
        .execute(&mut *conn)
        .await
        .context(format!(
            r#"failed to upsert "{}" into [share]"#,
            share.id().as_uuid()
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
            "DELETE FROM share
             WHERE id = $1",
        )
        .bind(id)
        .execute(&mut *conn)
        .await
        .context(format!(
            r#"failed to delete "{}" from [share]"#,
            id.as_uuid()
        ))
    }

    async fn select(
        &self,
        limit: Option<&i64>,
        after: Option<&Name>,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<Vec<Row>> {
        let mut conn = executor
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        let mut builder = QueryBuilder::new(
            "SELECT
                 id,
                 name,
                 created_by,
                 created_at,
                 updated_at
             FROM share",
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
        let mut query = sqlx::query_as::<_, Row>(builder.build().sql().into());
        if let Some(name) = after {
            query = query.bind(name);
        }
        if let Some(limit) = limit {
            query = query.bind(limit);
        }
        let rows: Vec<Row> = query
            .fetch_all(&mut *conn)
            .await
            .context("failed to list shares from [share]")?;
        Ok(rows)
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
                 created_by,
                 created_at,
                 updated_at
             FROM share
             WHERE name = $1",
        )
        .bind(name)
        .fetch_optional(&mut *conn)
        .await
        .context(format!(
            r#"failed to select "{}" from [share]"#,
            name.as_str()
        ))?;
        Ok(row)
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
            testutils::rand::i64(1, 100000),
        )
        .context("failed to upsert account")?;
        repo.upsert(&account, tx)
            .await
            .context("failed to insert account")?;
        Ok(account)
    }

    async fn upsert_share(account_id: &AccountId, tx: &mut PgConnection) -> Result<Entity> {
        let repo = PgRepository;
        let share = Entity::new(
            testutils::rand::uuid(),
            testutils::rand::string(10),
            account_id.to_uuid().to_string(),
        )
        .context("failed to upsert share")?;
        repo.upsert(&share, tx)
            .await
            .context("failed to insert share")?;
        Ok(share)
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
            upsert_share(account.id(), &mut tx)
                .await
                .expect("new share should be created");
        }
        let fetched = repo
            .select(None, None, &mut tx)
            .await
            .expect("inserted share should be listed");
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
            upsert_share(account.id(), &mut tx)
                .await
                .expect("new share should be created");
        }
        let limit = testutils::rand::i64(0, 20);
        let fetched = repo
            .select(Some(&limit), None, &mut tx)
            .await
            .expect("inserted share should be listed");
        assert_eq!(min(records, limit) as usize, fetched.len());
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
        let account = upsert_account(&mut tx)
            .await
            .expect("new account should be created");
        let share = upsert_share(account.id(), &mut tx)
            .await
            .expect("new share should be created");
        let fetched = repo
            .select_by_name(&share.name(), &mut tx)
            .await
            .expect("inserted share should be found");
        if let Some(fetched) = fetched {
            assert_eq!(&fetched.id, share.id().as_uuid());
            assert_eq!(&fetched.name, share.name().as_str());
            assert_eq!(&fetched.created_by, share.created_by().as_uuid());
        } else {
            panic!("inserted account should be found");
        }
        tx.rollback()
            .await
            .expect("rollback should be done properly");
        Ok(())
    }
}
