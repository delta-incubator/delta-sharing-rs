#![cfg(feature = "integration-pg")]

use std::sync::Arc;

use delta_sharing_common::{AssociationLabel, ObjectLabel};
use delta_sharing_postgres::{Error, GraphStore};
use uuid::Uuid;

#[sqlx::test]
async fn test_objects(pool: sqlx::PgPool) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let store = GraphStore::new(Arc::new(pool));

    let object = store
        .add_object(
            &ObjectLabel::SharingTable,
            &["namespace".to_string(), "table_name".to_string()],
            Some(serde_json::json!({ "key": "value" })),
        )
        .await?;
    assert_eq!(object.label, ObjectLabel::SharingTable);
    assert_eq!(
        object.name,
        vec!["namespace".to_string(), "table_name".to_string()].into()
    );

    // Adding the same object should fail.
    let res = store
        .add_object(
            &ObjectLabel::SharingTable,
            &["namespace".to_string(), "table_name".to_string()],
            Some(serde_json::json!({ "key": "value" })),
        )
        .await;
    assert!(matches!(res, Err(Error::AlreadyExists(_))));

    let object = store.get_object(&object.id).await?;
    assert_eq!(object.label, ObjectLabel::SharingTable);
    assert_eq!(
        object.name,
        vec!["namespace".to_string(), "table_name".to_string()].into()
    );
    assert_eq!(
        object.properties,
        Some(serde_json::json!({ "key": "value" }))
    );

    // Test get_object_by_name
    let object = store
        .get_object_by_name(
            &ObjectLabel::SharingTable,
            &["namespace".to_string(), "table_name".to_string()],
        )
        .await?;
    assert_eq!(object.label, ObjectLabel::SharingTable);
    assert_eq!(
        object.name,
        vec!["namespace".to_string(), "table_name".to_string()].into()
    );

    let object = store
        .update_object(
            &object.id,
            None,
            None,
            serde_json::json!({ "key": "value2" }),
        )
        .await?;
    assert_eq!(
        object.properties,
        Some(serde_json::json!({ "key": "value2" }))
    );

    // Updating an object with a non-existent ID should fail.
    let res = store
        .update_object(&Uuid::new_v4(), None, None, serde_json::json!({}))
        .await;
    assert!(matches!(res, Err(Error::EntityNotFound(_))));

    store.delete_object(&object.id).await?;
    let res = store.get_object(&object.id).await;
    assert!(matches!(res, Err(Error::EntityNotFound(_))));

    Ok(())
}

#[sqlx::test]
async fn test_associations(pool: sqlx::PgPool) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let store = GraphStore::new(Arc::new(pool));

    let object1 = store
        .add_object(
            &ObjectLabel::SharingTable,
            &["namespace".to_string(), "table_name1".to_string()],
            Some(serde_json::json!({ "key": "value" })),
        )
        .await?;
    let object2 = store
        .add_object(
            &ObjectLabel::SharingTable,
            &["namespace".to_string(), "table_name2".to_string()],
            Some(serde_json::json!({ "key": "value" })),
        )
        .await?;

    let association = store
        .add_association(
            &object1.id,
            &AssociationLabel::HasPart,
            &object2.id,
            serde_json::json!({ "key": "value" }),
        )
        .await?;
    assert_eq!(association.label, AssociationLabel::HasPart);
    assert_eq!(association.from_id, object1.id);
    assert_eq!(association.to_id, object2.id);

    // Adding the same association should fail.
    let res = store
        .add_association(
            &object1.id,
            &AssociationLabel::HasPart,
            &object2.id,
            serde_json::json!({ "key": "value" }),
        )
        .await;
    assert!(matches!(res, Err(Error::AlreadyExists(_))));

    // Adding an association with a non-existent source object should fail.
    let res = store
        .add_association(
            &Uuid::new_v4(),
            &AssociationLabel::HasPart,
            &object2.id,
            serde_json::json!({}),
        )
        .await;
    println!("{:?}", res);
    assert!(matches!(res, Err(Error::EntityNotFound(_))));

    // Adding an association with a non-existent target object should fail.
    let res = store
        .add_association(
            &object1.id,
            &AssociationLabel::HasPart,
            &Uuid::new_v4(),
            serde_json::json!({}),
        )
        .await;
    println!("{:?}", res);
    assert!(matches!(res, Err(Error::EntityNotFound(_))));

    let associations = store
        .get_associations(
            &object1.id,
            &AssociationLabel::HasPart,
            &[object2.id],
            None,
            None,
        )
        .await?;
    assert_eq!(associations.0.len(), 1);
    assert_eq!(associations.0[0].label, AssociationLabel::HasPart);
    assert_eq!(associations.0[0].from_id, object1.id);
    assert_eq!(associations.0[0].to_id, object2.id);

    // assert inverse association
    let associations = store
        .get_associations(
            &object2.id,
            &AssociationLabel::PartOf,
            &[object1.id],
            None,
            None,
        )
        .await?;
    assert_eq!(associations.0.len(), 1);
    assert_eq!(associations.0[0].label, AssociationLabel::PartOf);
    assert_eq!(associations.0[0].from_id, object2.id);
    assert_eq!(associations.0[0].to_id, object1.id);

    store
        .delete_association(&object1.id, &AssociationLabel::HasPart, &object2.id)
        .await?;
    let associations = store
        .get_associations(
            &object1.id,
            &AssociationLabel::HasPart,
            &[object2.id],
            None,
            None,
        )
        .await?;
    assert!(associations.0.is_empty());

    Ok(())
}
