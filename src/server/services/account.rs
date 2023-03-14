use crate::server::entities::account::Account;
use crate::server::entities::account::AccountId;
use crate::server::repositories::account::AccountRepository;
use crate::server::repositories::account::AccountRow;
use crate::server::repositories::account::PgAccountRepository;
use anyhow::Result;
use async_trait::async_trait;
use sqlx::postgres::PgQueryResult;
use sqlx::PgPool;

#[async_trait]
pub trait AccountService {
    async fn create(&self, account: &Account) -> Result<PgQueryResult>;

    async fn delete(&self, id: &AccountId) -> Result<PgQueryResult>;

    async fn get_by_id(&self, id: &AccountId) -> Result<Option<AccountRow>>;
}

#[async_trait]
impl AccountService for PgPool {
    async fn create(&self, account: &Account) -> Result<PgQueryResult> {
        let repo = PgAccountRepository;
        repo.create(account, self).await
    }

    async fn delete(&self, id: &AccountId) -> Result<PgQueryResult> {
        let repo = PgAccountRepository;
        repo.delete(id, self).await
    }

    async fn get_by_id(&self, id: &AccountId) -> Result<Option<AccountRow>> {
        let repo = PgAccountRepository;
        repo.get_by_id(id, self).await
    }
}
