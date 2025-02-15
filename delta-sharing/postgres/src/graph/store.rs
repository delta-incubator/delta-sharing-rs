use std::sync::Arc;

use sqlx::PgPool;
use uuid::Uuid;

use super::{Association, AssociationLabel, Object, ObjectLabel};
use crate::constants::MAX_PAGE_SIZE;
use crate::pagination::V1PaginateToken;
use crate::{error::Result, pagination::PaginateToken};

pub struct Store {
    pool: Arc<PgPool>,
}

impl Store {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn connect(url: impl AsRef<str>) -> Result<Self> {
        let pool = PgPool::connect(url.as_ref()).await.unwrap();
        Ok(Self::new(Arc::new(pool)))
    }

    /// Add an object to the store.
    pub async fn add_object(
        &self,
        label: &ObjectLabel,
        namespace: &[String],
        name: impl AsRef<str>,
        properties: impl Into<Option<serde_json::Value>>,
    ) -> Result<Object> {
        Ok(sqlx::query_as!(
            Object,
            r#"
            INSERT INTO objects ( label, namespace, name, properties )
            VALUES ( $1, $2, $3, $4 )
            RETURNING
                id,
                label AS "label: ObjectLabel",
                namespace,
                name,
                properties,
                created_at,
                updated_at
            "#,
            label as &ObjectLabel,
            namespace,
            name.as_ref(),
            properties.into()
        )
        .fetch_one(&*self.pool)
        .await?)
    }

    /// Get an object from the store.
    pub async fn get_object(&self, id: &Uuid) -> Result<Object> {
        Ok(sqlx::query_as!(
            Object,
            r#"
            SELECT
                id,
                label AS "label: ObjectLabel",
                namespace,
                name,
                properties,
                created_at,
                updated_at
            FROM objects
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&*self.pool)
        .await?)
    }

    /// Update an object in the store.
    pub async fn update_object(
        &self,
        id: &Uuid,
        properties: impl Into<Option<serde_json::Value>>,
    ) -> Result<Object> {
        Ok(sqlx::query_as!(
            Object,
            r#"
            UPDATE objects
            SET properties = $2
            WHERE id = $1
            RETURNING
                id,
                label AS "label: ObjectLabel",
                namespace,
                name,
                properties,
                created_at,
                updated_at
            "#,
            id,
            properties.into()
        )
        .fetch_one(&*self.pool)
        .await?)
    }

    /// Delete an object from the store.
    pub async fn delete_object(&self, id: &Uuid) -> Result<()> {
        let mut txn = self.pool.begin().await?;

        // Delete the associations.
        sqlx::query!(
            r#"
            DELETE FROM associations
            WHERE from_id = $1 OR to_id = $1
            "#,
            id
        )
        .execute(&mut *txn)
        .await?;

        // Delete the object.
        sqlx::query!(
            r#"
            DELETE FROM objects
            WHERE id = $1
            "#,
            id
        )
        .execute(&mut *txn)
        .await?;

        Ok(txn.commit().await?)
    }

    /// Add an association to the store.
    ///
    /// Associations are directed edges between objects.
    /// If an inverse association exists, it is automatically created.
    pub async fn add_association(
        &self,
        from_id: &Uuid,
        label: &AssociationLabel,
        to_id: &Uuid,
        properties: impl Into<Option<serde_json::Value>>,
    ) -> Result<Association> {
        let mut txn = self.pool.begin().await?;
        let properties = properties.into();

        // Add the association.
        let association = sqlx::query_as!(
            Association,
            r#"
            INSERT INTO associations ( from_id, label, to_id, properties )
            VALUES ( $1, $2, $3, $4 )
            RETURNING
                id,
                from_id,
                label AS "label: AssociationLabel",
                to_id,
                properties,
                created_at,
                updated_at
            "#,
            from_id,
            label as &AssociationLabel,
            to_id,
            properties.clone()
        )
        .fetch_one(&mut *txn)
        .await?;

        // Add the inverse association.
        if let Some(inverse_label) = label.inverse() {
            sqlx::query!(
                r#"
                INSERT INTO associations ( from_id, label, to_id, properties )
                VALUES ( $1, $2, $3, $4 )
                "#,
                to_id,
                inverse_label as AssociationLabel,
                from_id,
                properties
            )
            .execute(&mut *txn)
            .await?;
        }

        txn.commit().await?;

        Ok(association)
    }

    /// Delete an association from the store.
    ///
    /// If an inverse association exists, it is automatically deleted.
    pub async fn delete_association(
        &self,
        from_id: &Uuid,
        label: &AssociationLabel,
        to_id: &Uuid,
    ) -> Result<()> {
        let mut txn = self.pool.begin().await?;
        delete_association(from_id, label, to_id, &mut txn).await?;
        txn.commit().await?;
        Ok(())
    }

    /// List associations of a specific type from an object to a set of objects.
    pub async fn get_associations(
        &self,
        from_id: &Uuid,
        label: &AssociationLabel,
        to_ids: &[Uuid],
    ) -> Result<Vec<Association>> {
        Ok(sqlx::query_as!(
            Association,
            r#"
            SELECT
                id,
                from_id,
                label AS "label: AssociationLabel",
                to_id,
                properties,
                created_at,
                updated_at
            FROM associations
            WHERE from_id = $1
              AND label = $2
              AND to_id = ANY($3)
            ORDER BY created_at DESC
            LIMIT 1000
            "#,
            from_id,
            label as &AssociationLabel,
            &to_ids
        )
        .fetch_all(&*self.pool)
        .await?)
    }

    /// List associations of a specific type from an object to all objects.
    pub async fn list_associations(
        &self,
        from_id: &Uuid,
        label: &AssociationLabel,
        page_token: Option<&str>,
        max_page_size: Option<usize>,
    ) -> Result<(Vec<Association>, Option<String>)> {
        let max_page_size = usize::min(max_page_size.unwrap_or(MAX_PAGE_SIZE), MAX_PAGE_SIZE);
        let token = page_token
            .map(PaginateToken::<Uuid>::try_from)
            .transpose()?;
        let (_token_ts, token_id) = token
            .as_ref()
            .map(
                |PaginateToken::V1(V1PaginateToken { created_at, id }): &PaginateToken<Uuid>| {
                    (created_at, id)
                },
            )
            .unzip();

        let assocs = sqlx::query_as!(
            Association,
            r#"
            SELECT
                id,
                from_id,
                label AS "label: AssociationLabel",
                to_id,
                properties,
                created_at,
                updated_at
            FROM associations
            WHERE from_id = $1
              AND label = $2
              -- Pagination
              AND ( id < $3 OR $3 IS NULL )
            ORDER BY id DESC
            LIMIT $4
            "#,
            from_id,
            label as &AssociationLabel,
            token_id,
            max_page_size as i64
        )
        .fetch_all(&*self.pool)
        .await?;

        let next = (assocs.len() == max_page_size)
            .then(|| {
                assocs.last().map(|a| {
                    PaginateToken::V1(V1PaginateToken {
                        created_at: a.created_at,
                        id: a.id,
                    })
                    .to_string()
                })
            })
            .flatten();

        Ok((assocs, next))
    }
}

async fn delete_association(
    from_id: &Uuid,
    label: &AssociationLabel,
    to_id: &Uuid,
    txn: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> Result<(), crate::error::Error> {
    sqlx::query!(
        r#"
        DELETE FROM associations
        WHERE from_id = $1 AND label = $2 AND to_id = $3
        "#,
        from_id,
        label as &AssociationLabel,
        to_id
    )
    .execute(&mut **txn)
    .await?;
    if let Some(inverse_label) = label.inverse() {
        sqlx::query!(
            r#"
            DELETE FROM associations
            WHERE from_id = $1 AND label = $2 AND to_id = $3
            "#,
            to_id,
            inverse_label as AssociationLabel,
            from_id
        )
        .execute(&mut **txn)
        .await?;
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Error;

    #[sqlx::test]
    async fn test_objects(pool: sqlx::PgPool) -> Result<(), Box<dyn std::error::Error + 'static>> {
        let store = Store::new(Arc::new(pool));

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
    async fn test_associations(
        pool: sqlx::PgPool,
    ) -> Result<(), Box<dyn std::error::Error + 'static>> {
        let store = Store::new(Arc::new(pool));

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
}
