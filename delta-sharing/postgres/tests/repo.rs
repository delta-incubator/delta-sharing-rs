#![cfg(feature = "integration-pg")]

use delta_sharing_postgres::{PgSharingRepo, SharingRepo};

#[sqlx::test]
async fn test_tables(pool: sqlx::PgPool) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let repo = PgSharingRepo::new(pool);

    let mut record = repo.add_table("table_name", "file:///location").await?;
    let retrieved = repo.get_table(&record.id).await?;
    assert_eq!(record.id, retrieved.id);

    record.location = url::Url::parse("file:///location-new").unwrap();
    let updated = repo.update_table(&record).await?;
    assert_eq!(record.location, updated.location);

    let retrieved = repo.get_table(&record.id).await?;
    assert_eq!(record.location, retrieved.location);

    Ok(())
}
