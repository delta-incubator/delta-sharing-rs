use crate::server::entities::account::Account;
use crate::server::entities::account::AccountId;
use crate::wrappers::postgres::PgAcquire;
use anyhow::Context;
use anyhow::Result;
use async_trait::async_trait;
use chrono::DateTime;
use chrono::Utc;
use sqlx::postgres::PgQueryResult;
use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow)]
pub struct AccountRow {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub namespace: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[async_trait]
pub trait AccountRepository: Send + Sync + 'static {
    async fn create(
        &self,
        account: &Account,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<PgQueryResult>;

    async fn delete(
        &self,
        id: &AccountId,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<PgQueryResult>;

    async fn get_by_id(
        &self,
        id: &AccountId,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<Option<AccountRow>>;
}

pub struct PgAccountRepository;

#[async_trait]
impl AccountRepository for PgAccountRepository {
    async fn create(
        &self,
        account: &Account,
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
        id: &AccountId,
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

    async fn get_by_id(
        &self,
        id: &AccountId,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<Option<AccountRow>> {
        let mut conn = executor
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        let row: Option<AccountRow> = sqlx::query_as::<_, AccountRow>(
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Context;
    use anyhow::Result;
    use sqlx::PgConnection;
    use sqlx::PgPool;

    async fn create_account(tx: &mut PgConnection) -> Result<Account> {
        let repo = PgAccountRepository;
        let account = Account::new(
            testutils::rand::uuid(),
            testutils::rand::string(10),
            testutils::rand::email(),
            testutils::rand::string(10),
            testutils::rand::string(10),
        )
        .context("failed to create account")?;
        repo.create(&account, tx)
            .await
            .context("failed to insert account")?;
        Ok(account)
    }

    #[sqlx::test]
    #[ignore] // NOTE: Be sure '$ docker compose -f devops/local/docker-compose.yaml up' before running this test
    async fn test_create_and_get_by_id(pool: PgPool) -> Result<()> {
        let repo = PgAccountRepository;
        let mut tx = pool
            .begin()
            .await
            .expect("transaction should be started properly");
        let account = create_account(&mut tx)
            .await
            .expect("new account should be created");
        let fetched = repo
            .get_by_id(&account.id(), &mut tx)
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
