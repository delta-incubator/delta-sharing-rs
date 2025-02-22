#![cfg(feature = "integration-pg")]

use std::sync::Arc;

use delta_sharing_common::{Error, ResourceRef, SharingRepository};
use uuid::Uuid;

use delta_sharing_postgres::*;

#[sqlx::test]
async fn test_shares(pool: sqlx::PgPool) {
    let repo = GraphStore::new(Arc::new(pool));
    let share = repo
        .add_share("test_share", Some("test comment".to_string()), None)
        .await
        .unwrap();
    assert_eq!(share.name, "test_share");

    let share = repo
        .get_share(&ResourceRef::name(vec!["test_share".to_string()]))
        .await
        .unwrap();
    assert_eq!(share.name, "test_share");

    let share = repo
        .get_share(&ResourceRef::Uuid(share.id.parse().unwrap()))
        .await
        .unwrap();
    assert_eq!(share.name, "test_share");

    // test get share with unknown id should fail
    let res = repo.get_share(&ResourceRef::Uuid(Uuid::new_v4())).await;
    assert!(matches!(res, Err(Error::NotFound)));

    // test get share with unknown name should fail
    let res = repo
        .get_share(&ResourceRef::name(vec!["unknown".to_string()]))
        .await;
    assert!(matches!(res, Err(Error::NotFound)));

    repo.delete_share(&ResourceRef::name(vec!["test_share".to_string()]))
        .await
        .unwrap();
    let res = repo
        .get_share(&ResourceRef::name(vec!["test_share".to_string()]))
        .await;
    assert!(matches!(res, Err(Error::NotFound)));

    // list shares
    let shares = repo.list_shares(None, None).await.unwrap();
    assert_eq!(shares.0.len(), 0);

    let _share = repo
        .add_share("test_share", Some("test comment".to_string()), None)
        .await
        .unwrap();
    let shares = repo.list_shares(None, None).await.unwrap();
    assert_eq!(shares.0.len(), 1);
    assert_eq!(shares.0[0].name, "test_share");

    // sleep to ensure the next share has a different created_at
    tokio::time::sleep(std::time::Duration::from_millis(2)).await;

    let _share = repo
        .add_share("test_share2", Some("test comment".to_string()), None)
        .await
        .unwrap();
    let shares = repo.list_shares(Some(1), None).await.unwrap();
    assert_eq!(shares.0.len(), 1);
    assert_eq!(shares.0[0].name, "test_share2");
}

#[sqlx::test]
async fn test_schema(pool: sqlx::PgPool) {
    let repo = GraphStore::new(Arc::new(pool));
    let share = repo
        .add_share("test_share", Some("test comment".to_string()), None)
        .await
        .unwrap();

    let schema = repo
        .add_schema(
            &ResourceRef::name(vec![share.name.clone()]),
            "test_schema",
            Some("test comment".to_string()),
            None,
        )
        .await
        .unwrap();
    assert_eq!(schema.share, share.name);
    assert_eq!(schema.name, "test_schema");

    let schema = repo
        .get_schema(&ResourceRef::name(vec![
            share.name.clone(),
            "test_schema".to_string(),
        ]))
        .await
        .unwrap();
    assert_eq!(schema.share, share.name);
    assert_eq!(schema.name, "test_schema");

    // let schema = repo
    //     .get_schema(&SchemaRef::Uuid(schema.id.unwrap().parse().unwrap()))
    //     .await
    //     .unwrap();
    // assert_eq!(schema.share, share.name);
    // assert_eq!(schema.name, "test_schema");

    // test get schema with unknown id should fail
    let res = repo.get_schema(&ResourceRef::Uuid(Uuid::new_v4())).await;
    assert!(matches!(res, Err(Error::NotFound)));

    // test get schema with unknown name should fail
    let res = repo
        .get_schema(&ResourceRef::name(vec![
            share.name.clone(),
            "unknown".to_string(),
        ]))
        .await;
    assert!(matches!(res, Err(Error::NotFound)));

    repo.delete_schema(&ResourceRef::name(vec![
        share.name.clone(),
        "test_schema".to_string(),
    ]))
    .await
    .unwrap();
    let res = repo
        .get_schema(&ResourceRef::name(vec![
            share.name.clone(),
            "test_schema".to_string(),
        ]))
        .await;
    assert!(matches!(res, Err(Error::NotFound)));

    // list schemas
    let schemas = repo
        .list_schemas(&ResourceRef::name(vec![share.name.clone()]), None, None)
        .await
        .unwrap();
    assert_eq!(schemas.0.len(), 0);

    let _schema = repo
        .add_schema(
            &ResourceRef::name(vec![share.name.clone()]),
            "test_schema",
            Some("test comment".to_string()),
            None,
        )
        .await
        .unwrap();
    let schemas = repo
        .list_schemas(&ResourceRef::name(vec![share.name]), None, None)
        .await
        .unwrap();
    assert_eq!(schemas.0.len(), 1);
    assert_eq!(schemas.0[0].name, "test_schema");
}
