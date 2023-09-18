use std::str::FromStr;

use anyhow::Context;
use anyhow::Result;
use sqlx::PgConnection;

use delta_sharing::server::Role;
use delta_sharing::server::{AccountEntity, AccountId, AccountRepository};
use delta_sharing::server::{SchemaEntity, SchemaId, SchemaRepository};
use delta_sharing::server::{ShareEntity, ShareId, ShareRepository};
use delta_sharing::server::{TableEntity, TableRepository};
use delta_sharing::server::{TokenEntity, TokenRepository};

pub async fn create_account(tx: &mut PgConnection) -> Result<AccountEntity> {
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

pub async fn create_token(account_id: &AccountId, tx: &mut PgConnection) -> Result<TokenEntity> {
    let roles = vec!["Admin", "Guest"];
    let role = testutils::rand::choose(&roles);
    let role = Role::from_str(role).context("failed to choose role")?;
    let token = TokenEntity::new(
        testutils::rand::uuid(),
        testutils::rand::email(),
        role,
        testutils::rand::string(10),
        account_id.to_uuid().to_string(),
    )
    .context("failed to validate token")?;
    TokenRepository::upsert(&token, tx)
        .await
        .context("failed to create token")?;
    Ok(token)
}

pub async fn create_share(account_id: &AccountId, tx: &mut PgConnection) -> Result<ShareEntity> {
    let share = ShareEntity::new(
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

pub async fn create_schema(
    account_id: &AccountId,
    share_id: &ShareId,
    tx: &mut PgConnection,
) -> Result<SchemaEntity> {
    let schema = SchemaEntity::new(
        testutils::rand::uuid(),
        testutils::rand::string(10),
        share_id.to_uuid().to_string(),
        account_id.to_uuid().to_string(),
    )
    .context("failed to validate schema")?;
    SchemaRepository::upsert(&schema, tx)
        .await
        .context("failed to create schema")?;
    Ok(schema)
}

pub async fn create_table(
    account_id: &AccountId,
    schema_id: &SchemaId,
    tx: &mut PgConnection,
) -> Result<TableEntity> {
    let table = TableEntity::new(
        testutils::rand::uuid(),
        testutils::rand::string(10),
        schema_id.to_uuid().to_string(),
        testutils::rand::string(10),
        account_id.to_uuid().to_string(),
    )
    .context("failed to validate table")?;
    TableRepository::upsert(&table, tx)
        .await
        .context("failed to create table")?;
    Ok(table)
}
