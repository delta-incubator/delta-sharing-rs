// NOTE: Be sure '$ docker compose -f devops/local/docker-compose.yaml up'
// before running these tests
use std::str::FromStr;

use anyhow::Context;
use anyhow::Result;
use sqlx::PgConnection;
use sqlx::PgPool;

use delta_sharing::server::Role;
use delta_sharing::server::{AccountEntity, AccountId, AccountRepository};
use delta_sharing::server::{SchemaEntity, SchemaId, SchemaRepository};
use delta_sharing::server::{ShareEntity, ShareId, ShareRepository};
use delta_sharing::server::{TableEntity, TableRepository};
use delta_sharing::server::{TokenEntity, TokenRepository};

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

async fn create_token(account_id: &AccountId, tx: &mut PgConnection) -> Result<TokenEntity> {
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

async fn create_share(account_id: &AccountId, tx: &mut PgConnection) -> Result<ShareEntity> {
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

async fn create_schema(
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

async fn create_table(
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

#[sqlx::test]
async fn test_account_create_and_select_by_name(pool: PgPool) -> Result<()> {
    let mut tx = pool
        .begin()
        .await
        .expect("transaction should be started properly");
    let account = create_account(&mut tx)
        .await
        .expect("new account should be created");
    let fetched = AccountRepository::select_by_name(account.name(), &mut tx)
        .await
        .expect("created account should be found");
    assert!(fetched.is_some());

    let fetched = fetched.unwrap();
    assert_eq!(&fetched.id, account.id().as_uuid());
    assert_eq!(&fetched.name, account.name().as_str());
    assert_eq!(&fetched.email, account.email().as_str());
    assert_eq!(&fetched.password, account.password().as_str());
    assert_eq!(&fetched.namespace, account.namespace().as_str());
    assert_eq!(&fetched.ttl, account.ttl().as_i64());

    tx.rollback()
        .await
        .expect("rollback should be done properly");
    Ok(())
}

#[sqlx::test]
async fn test_token_create(pool: PgPool) -> Result<()> {
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

#[sqlx::test]
async fn test_share_create_and_select_by_name(pool: PgPool) -> Result<()> {
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
    let fetched = ShareRepository::select_by_name(share.name(), &mut tx)
        .await
        .expect("created share should be found");
    assert!(fetched.is_some());

    let fetched = fetched.unwrap();
    assert_eq!(&fetched.id, share.id().as_uuid());
    assert_eq!(&fetched.name, share.name().as_str());
    assert_eq!(&fetched.created_by, share.created_by().as_uuid());

    tx.rollback()
        .await
        .expect("rollback should be done properly");
    Ok(())
}

#[sqlx::test]
async fn test_schema_create_and_select_by_name(pool: PgPool) -> Result<()> {
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
    let schema = create_schema(account.id(), share.id(), &mut tx)
        .await
        .expect("new schema should be created");

    let fetched = SchemaRepository::select_by_name(share.id(), schema.name(), &mut tx)
        .await
        .expect("created share should be found");
    assert!(fetched.is_some());

    let fetched = fetched.unwrap();
    assert_eq!(&fetched.id, schema.id().as_uuid());
    assert_eq!(&fetched.name, schema.name().as_str());
    assert_eq!(&fetched.created_by, schema.created_by().as_uuid());

    tx.rollback()
        .await
        .expect("rollback should be done properly");
    Ok(())
}

#[sqlx::test]
async fn test_table_create_and_select_by_name(pool: PgPool) -> Result<()> {
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
    let schema = create_schema(account.id(), share.id(), &mut tx)
        .await
        .expect("new share should be created");
    let table = create_table(account.id(), schema.id(), &mut tx)
        .await
        .expect("new table should be created");
    let fetched = TableRepository::select_by_name(schema.id(), table.name(), &mut tx)
        .await
        .expect("created table should be found");
    assert!(fetched.is_some());

    let fetched = fetched.unwrap();
    assert_eq!(&fetched.id, table.id().as_uuid());
    assert_eq!(&fetched.name, table.name().as_str());
    assert_eq!(&fetched.location, table.location().as_str());
    assert_eq!(&fetched.created_by, table.created_by().as_uuid());

    tx.rollback()
        .await
        .expect("rollback should be done properly");
    Ok(())
}
