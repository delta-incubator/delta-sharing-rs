use anyhow::Context;
use anyhow::Result;
use chrono::DateTime;
use chrono::Utc;
use sqlx::postgres::PgQueryResult;
use uuid::Uuid;

use crate::server::entities::token::Entity;
use crate::server::middlewares::jwt::Role;
use crate::server::utilities::postgres::PgAcquire;

#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow)]
pub struct Row {
    pub id: Uuid,
    pub email: String,
    pub role: Role,
    pub value: String,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct Repository;

impl Repository {
    pub async fn upsert(token: &Entity, executor: impl PgAcquire<'_>) -> Result<PgQueryResult> {
        let mut conn = executor
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        sqlx::query(
            r#"INSERT INTO token (
                   id,
                   email,
                   "role",
                   "value",
                   created_by
               ) VALUES ($1, $2, $3, $4, $5)
               ON CONFLICT(id)
               DO UPDATE
               SET email = $2,
                   "role" = $3,
                   "value" = $4,
                   created_by = $5"#,
        )
        .bind(token.id())
        .bind(token.email())
        .bind(token.role())
        .bind(token.value())
        .bind(token.created_by())
        .execute(&mut *conn)
        .await
        .context(format!(
            r#"failed to upsert "{}" into [token]"#,
            token.id().as_uuid()
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::entities::account::Entity as Account;
    use crate::server::entities::account::Id as AccountId;
    use crate::server::repositories::account::Repository as AccountRepository;
    use anyhow::Context;
    use anyhow::Result;
    use sqlx::PgConnection;
    use sqlx::PgPool;
    use std::str::FromStr;

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

    async fn create_token(account_id: &AccountId, tx: &mut PgConnection) -> Result<Entity> {
        let roles = vec!["Admin", "Guest"];
        let role = testutils::rand::choose(&roles);
        let role = Role::from_str(role).context("failed to choose role")?;
        let token = Entity::new(
            testutils::rand::uuid(),
            testutils::rand::email(),
            role,
            testutils::rand::string(10),
            account_id.to_uuid().to_string(),
        )
        .context("failed to validate token")?;
        Repository::upsert(&token, tx)
            .await
            .context("failed to create token")?;
        Ok(token)
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
        create_token(account.id(), &mut tx)
            .await
            .expect("new token should be created");
        tx.rollback()
            .await
            .expect("rollback should be done properly");
        Ok(())
    }
}
