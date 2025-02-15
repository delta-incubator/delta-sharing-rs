use std::sync::Arc;

use delta_sharing_postgres::{AssociationLabel, Error, GraphStore, ObjectLabel};

#[sqlx::test]
async fn test_objects(pool: sqlx::PgPool) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let store = GraphStore::new(Arc::new(pool));

    let object = store
        .add_object(
            &ObjectLabel::Table,
            &["namespace".to_string()],
            "table_name",
            serde_json::json!({ "key": "value" }),
        )
        .await?;
    assert_eq!(object.label, ObjectLabel::Table);
    assert_eq!(object.namespace, vec!["namespace".to_string()]);
    assert_eq!(object.name, "table_name");

    let object = store.get_object(&object.id).await?;
    assert_eq!(object.label, ObjectLabel::Table);
    assert_eq!(object.namespace, vec!["namespace".to_string()]);
    assert_eq!(object.name, "table_name");
    assert_eq!(
        object.properties,
        Some(serde_json::json!({ "key": "value" }))
    );

    let object = store
        .update_object(&object.id, serde_json::json!({ "key": "value2" }))
        .await?;
    assert_eq!(
        object.properties,
        Some(serde_json::json!({ "key": "value2" }))
    );

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
            &ObjectLabel::Table,
            &["namespace".to_string()],
            "table_name1",
            serde_json::json!({ "key": "value" }),
        )
        .await?;
    let object2 = store
        .add_object(
            &ObjectLabel::Table,
            &["namespace".to_string()],
            "table_name2",
            serde_json::json!({ "key": "value" }),
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

    let associations = store
        .get_associations(&object1.id, &AssociationLabel::HasPart, &[object2.id])
        .await?;
    assert_eq!(associations.len(), 1);
    assert_eq!(associations[0].label, AssociationLabel::HasPart);
    assert_eq!(associations[0].from_id, object1.id);
    assert_eq!(associations[0].to_id, object2.id);

    // assert inverse association
    let associations = store
        .get_associations(&object2.id, &AssociationLabel::PartOf, &[object1.id])
        .await?;
    assert_eq!(associations.len(), 1);
    assert_eq!(associations[0].label, AssociationLabel::PartOf);
    assert_eq!(associations[0].from_id, object2.id);
    assert_eq!(associations[0].to_id, object1.id);

    store
        .delete_association(&object1.id, &AssociationLabel::HasPart, &object2.id)
        .await?;
    let associations = store
        .get_associations(&object1.id, &AssociationLabel::HasPart, &[object2.id])
        .await?;
    assert!(associations.is_empty());

    Ok(())
}
