// NOTE: Be sure '$ docker compose -f devops/local/docker-compose.yaml up'
// before running these tests
mod common;

use std::cmp::min;

use anyhow::Result;
use sqlx::PgPool;

use delta_sharing_legacy::server::AccountService;
use delta_sharing_legacy::server::SchemaService;
use delta_sharing_legacy::server::ShareService;
use delta_sharing_legacy::server::TableService;

use common::{create_account, create_schema, create_share, create_table};

#[sqlx::test]
async fn test_account_create_and_query_with_default_limit(pool: PgPool) -> Result<()> {
    let mut tx = pool
        .begin()
        .await
        .expect("transaction should be started properly");
    let records = testutils::rand::i64(0, 20);
    for _ in 0..records {
        create_account(&mut tx)
            .await
            .expect("new account should be created");
    }
    let fetched = AccountService::query(None, None, &mut tx)
        .await
        .expect("created account should be listed");
    assert_eq!(records as usize, fetched.len());
    tx.rollback()
        .await
        .expect("rollback should be done properly");
    Ok(())
}

#[sqlx::test]
async fn test_account_create_and_query_with_specified_limit(pool: PgPool) -> Result<()> {
    let mut tx = pool
        .begin()
        .await
        .expect("transaction should be started properly");
    let records = testutils::rand::i64(0, 20);
    for _ in 0..records {
        create_account(&mut tx)
            .await
            .expect("new account should be created");
    }
    let limit = testutils::rand::i64(0, 20);
    let fetched = AccountService::query(Some(&limit), None, &mut tx)
        .await
        .expect("created account should be listed");
    assert_eq!(min(records, limit) as usize, fetched.len());
    tx.rollback()
        .await
        .expect("rollback should be done properly");
    Ok(())
}

#[sqlx::test]
async fn test_account_create_and_query_by_name(pool: PgPool) -> Result<()> {
    let mut tx = pool
        .begin()
        .await
        .expect("transaction should be started properly");
    let account = create_account(&mut tx)
        .await
        .expect("new account should be created");
    let fetched = AccountService::query_by_name(account.name(), &mut tx)
        .await
        .expect("created account should be found");
    if let Some(fetched) = fetched {
        assert_eq!(&fetched.name, account.name().as_str());
        assert_eq!(&fetched.email, account.email().as_str());
        assert_eq!(&fetched.namespace, account.namespace().as_str());
        assert_eq!(&fetched.ttl, account.ttl().as_i64());
    } else {
        panic!("created account should be found");
    }
    tx.rollback()
        .await
        .expect("rollback should be done properly");
    Ok(())
}

#[sqlx::test]
async fn test_share_create_and_query_with_default_limit(pool: PgPool) -> Result<()> {
    let mut tx = pool
        .begin()
        .await
        .expect("transaction should be started properly");
    let account = create_account(&mut tx)
        .await
        .expect("new account should be created");
    let records = testutils::rand::i64(0, 20);
    for _ in 0..records {
        create_share(account.id(), &mut tx)
            .await
            .expect("new share should be created");
    }
    let fetched = ShareService::query(None, None, &mut tx)
        .await
        .expect("created share should be listed");
    assert_eq!(records as usize, fetched.len());
    tx.rollback()
        .await
        .expect("rollback should be done properly");
    Ok(())
}

#[sqlx::test]
async fn test_share_create_and_query_with_specified_limit(pool: PgPool) -> Result<()> {
    let mut tx = pool
        .begin()
        .await
        .expect("transaction should be started properly");
    let account = create_account(&mut tx)
        .await
        .expect("new account should be created");
    let records = testutils::rand::i64(0, 20);
    for _ in 0..records {
        create_share(account.id(), &mut tx)
            .await
            .expect("new share should be created");
    }
    let limit = testutils::rand::i64(0, 20);
    let fetched = ShareService::query(Some(&limit), None, &mut tx)
        .await
        .expect("created share should be listed");
    assert_eq!(min(records, limit) as usize, fetched.len());
    tx.rollback()
        .await
        .expect("rollback should be done properly");
    Ok(())
}

#[sqlx::test]
async fn test_share_create_and_query_by_name(pool: PgPool) -> Result<()> {
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
    let fetched = ShareService::query_by_name(share.name(), &mut tx)
        .await
        .expect("created share should be found");
    if let Some(fetched) = fetched {
        assert_eq!(&fetched.id, share.id().as_uuid().to_string().as_str());
        assert_eq!(&fetched.name, share.name().as_str());
    } else {
        panic!("created account should be found");
    }
    tx.rollback()
        .await
        .expect("rollback should be done properly");
    Ok(())
}

#[sqlx::test]
async fn test_schema_create_and_query_with_default_limit(pool: PgPool) -> Result<()> {
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
    let fetched = SchemaService::query_by_share_name(share.name(), None, None, &mut tx)
        .await
        .expect("created schema should be listed");
    assert_eq!(records as usize, fetched.len());
    tx.rollback()
        .await
        .expect("rollback should be done properly");
    Ok(())
}

#[sqlx::test]
async fn test_schema_create_and_query_with_specified_limit(pool: PgPool) -> Result<()> {
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
    let limit = testutils::rand::i64(0, 20);
    let fetched = SchemaService::query_by_share_name(share.name(), Some(&limit), None, &mut tx)
        .await
        .expect("created schema should be listed");
    assert_eq!(min(records, limit) as usize, fetched.len());
    tx.rollback()
        .await
        .expect("rollback should be done properly");
    Ok(())
}

#[sqlx::test]
async fn test_table_create_and_query_with_default_limit(pool: PgPool) -> Result<()> {
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
    let records = testutils::rand::i64(0, 20);
    for _ in 0..records {
        create_table(account.id(), schema.id(), &mut tx)
            .await
            .expect("new table should be created");
    }
    let fetched = TableService::query(None, None, &mut tx)
        .await
        .expect("created table should be listed");
    assert_eq!(records as usize, fetched.len());
    tx.rollback()
        .await
        .expect("rollback should be done properly");
    Ok(())
}

#[sqlx::test]
async fn test_table_create_and_query_with_specified_limit(pool: PgPool) -> Result<()> {
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
    let records = testutils::rand::i64(0, 20);
    for _ in 0..records {
        create_table(account.id(), schema.id(), &mut tx)
            .await
            .expect("new table should be created");
    }
    let limit = testutils::rand::i64(0, 20);
    let fetched = TableService::query(Some(&limit), None, &mut tx)
        .await
        .expect("created table should be listed");
    assert_eq!(min(records, limit) as usize, fetched.len());
    tx.rollback()
        .await
        .expect("rollback should be done properly");
    Ok(())
}

#[sqlx::test]
async fn test_table_create_and_query_by_name(pool: PgPool) -> Result<()> {
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
    let table = create_table(account.id(), schema.id(), &mut tx)
        .await
        .expect("new table should be created");
    let fetched = TableService::query_by_name(table.name(), &mut tx)
        .await
        .expect("created table should be found");
    if let Some(fetched) = fetched {
        assert_eq!(&fetched.id, table.id().as_uuid().to_string().as_str());
        assert_eq!(&fetched.name, table.name().as_str());
    } else {
        panic!("created table should be found");
    }
    tx.rollback()
        .await
        .expect("rollback should be done properly");
    Ok(())
}

#[sqlx::test]
async fn test_table_create_and_query_by_fqn(pool: PgPool) -> Result<()> {
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
    let table = create_table(account.id(), schema.id(), &mut tx)
        .await
        .expect("new table should be created");
    let fetched = TableService::query_by_fqn(share.name(), schema.name(), table.name(), &mut tx)
        .await
        .expect("created table should be found");
    if let Some(fetched) = fetched {
        assert_eq!(&fetched.id, table.id().as_uuid().to_string().as_str());
        assert_eq!(&fetched.name, table.name().as_str());
    } else {
        panic!("created table should be found");
    }
    tx.rollback()
        .await
        .expect("rollback should be done properly");
    Ok(())
}

#[sqlx::test]
async fn test_table_create_and_query_by_share_name_with_default_limit(pool: PgPool) -> Result<()> {
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
    let num_schemas = testutils::rand::i64(0, 20);
    let num_tables = testutils::rand::i64(0, 20);
    for _ in 0..num_schemas {
        let schema = create_schema(account.id(), share.id(), &mut tx)
            .await
            .expect("new schema should be created");
        for _ in 0..num_tables {
            create_table(account.id(), schema.id(), &mut tx)
                .await
                .expect("new schema should be created");
        }
    }
    let fetched = TableService::query_by_share_name(share.name(), None, None, &mut tx)
        .await
        .expect("created table should be listed");
    assert_eq!((num_schemas * num_tables) as usize, fetched.len());
    tx.rollback()
        .await
        .expect("rollback should be done properly");
    Ok(())
}

#[sqlx::test]
async fn test_table_create_and_query_by_share_name_with_specified_limit(
    pool: PgPool,
) -> Result<()> {
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
    let num_schemas = testutils::rand::i64(0, 20);
    let num_tables = testutils::rand::i64(0, 20);
    for _ in 0..num_schemas {
        let schema = create_schema(account.id(), share.id(), &mut tx)
            .await
            .expect("new schema should be created");
        for _ in 0..num_tables {
            create_table(account.id(), schema.id(), &mut tx)
                .await
                .expect("new table should be created");
        }
    }
    let limit = testutils::rand::i64(0, 20);
    let fetched = TableService::query_by_share_name(share.name(), Some(&limit), None, &mut tx)
        .await
        .expect("created schema should be listed");
    assert_eq!(min(num_schemas * num_tables, limit) as usize, fetched.len());
    tx.rollback()
        .await
        .expect("rollback should be done properly");
    Ok(())
}

#[sqlx::test]
async fn test_table_create_and_query_by_share_and_schema_name_with_default_limit(
    pool: PgPool,
) -> Result<()> {
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
    let records = testutils::rand::i64(0, 20);
    for _ in 0..records {
        create_table(account.id(), schema.id(), &mut tx)
            .await
            .expect("new schema should be created");
    }
    let fetched = TableService::query_by_share_and_schema_name(
        share.name(),
        schema.name(),
        None,
        None,
        &mut tx,
    )
    .await
    .expect("created table should be listed");
    assert_eq!(records as usize, fetched.len());
    tx.rollback()
        .await
        .expect("rollback should be done properly");
    Ok(())
}

#[sqlx::test]
async fn test_table_create_and_query_by_share_and_schema_name_with_specified_limit(
    pool: PgPool,
) -> Result<()> {
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
    let records = testutils::rand::i64(5, 20);
    for _ in 0..records {
        create_table(account.id(), schema.id(), &mut tx)
            .await
            .expect("new schema should be created");
    }
    let limit = testutils::rand::i64(0, 20);
    let fetched = TableService::query_by_share_and_schema_name(
        share.name(),
        schema.name(),
        Some(&limit),
        None,
        &mut tx,
    )
    .await
    .expect("created schema should be listed");
    assert_eq!(min(records, limit) as usize, fetched.len());
    tx.rollback()
        .await
        .expect("rollback should be done properly");
    Ok(())
}
