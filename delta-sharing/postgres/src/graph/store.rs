use std::sync::Arc;

use sqlx::migrate::Migrator;
use sqlx::PgPool;
use uuid::Uuid;

use super::{Association, AssociationLabel, Object, ObjectLabel};
use crate::constants::MAX_PAGE_SIZE;
use crate::pagination::V1PaginateToken;
use crate::{error::Result, pagination::PaginateToken};

static MIGRATOR: Migrator = sqlx::migrate!();

#[derive(Clone)]
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

    pub async fn migrate(&self) -> Result<()> {
        MIGRATOR.run(&*self.pool).await?;
        Ok(())
    }

    /// Add an object to the store.
    ///
    /// # Parameters
    /// - `label`: The label of the object.
    /// - `namespace`: The namespace of the object.
    /// - `name`: The name of the object.
    /// - `properties`: The properties of the object.
    ///
    /// # Returns
    /// The object that was added to the store.
    ///
    /// # Errors
    /// - [AlreadyExists](crate::Error::AlreadyExists): If an object with the
    ///   same name already exists in the namespace
    pub async fn add_object(
        &self,
        label: &ObjectLabel,
        namespace: &[String],
        name: impl AsRef<str>,
        properties: Option<serde_json::Value>,
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
            properties
        )
        .fetch_one(&*self.pool)
        .await?)
    }

    /// Get an object from the store.
    ///
    /// # Parameters
    /// - `id`: The globally unique identifier of the object.
    ///
    /// # Returns
    /// The object with the given identifier.
    ///
    /// # Errors
    /// - [EntityNotFound](crate::Error::EntityNotFound): If the object does not exist.
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

    /// Get an object from the store by name.
    ///
    /// The name of the object is unique within the namespace.
    ///
    /// # Parameters
    /// - `label`: The label of the object.
    /// - `namespace`: The namespace of the object.
    /// - `name`: The name of the object.
    ///
    /// # Returns
    /// The object with the given name.
    ///
    /// # Errors
    /// - [EntityNotFound](crate::Error::EntityNotFound): If the object does not exist.
    pub async fn get_object_by_name(
        &self,
        label: &ObjectLabel,
        namespace: &[String],
        name: impl AsRef<str>,
    ) -> Result<Object> {
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
            WHERE label = $1
              AND namespace = $2
              AND name = $3
            "#,
            label as &ObjectLabel,
            namespace,
            name.as_ref()
        )
        .fetch_one(&*self.pool)
        .await?)
    }

    /// Update an object in the store.
    ///
    /// # Parameters
    /// - `id`: The globally unique identifier of the object.
    /// - `properties`: The properties of the object.
    ///
    /// # Returns
    /// The updated object.
    ///
    /// # Errors
    /// - [EntityNotFound](crate::Error::EntityNotFound): If the object does not exist.
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
    ///
    /// # Parameters
    /// - `id`: The globally unique identifier of the object.
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

    /// List objects from the store.
    ///
    /// Returns a list of objects in the namespace. The list is paginated.
    ///
    /// # Parameters
    /// - `label`: The label of the objects.
    /// - `namespace`: The namespace of the objects.
    /// - `page_token`: The page token.
    /// - `max_page_size`: The maximum page size.
    ///
    /// # Returns
    /// A tuple containing the objects in the namespace and an optional next page token.
    pub async fn list_objects(
        &self,
        label: &ObjectLabel,
        namespace: &[String],
        page_token: Option<&str>,
        max_page_size: Option<usize>,
    ) -> Result<(Vec<Object>, Option<String>)> {
        let max_page_size = usize::min(max_page_size.unwrap_or(MAX_PAGE_SIZE), MAX_PAGE_SIZE);
        let token = page_token
            .map(PaginateToken::<Uuid>::try_from)
            .transpose()?;
        let (_token_ts, token_id) = token
            .as_ref()
            .map(|PaginateToken::V1(V1PaginateToken { created_at, id })| (created_at, id))
            .unzip();

        let objects = sqlx::query_as!(
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
            WHERE label = $1
              AND namespace = $2
              AND ( id < $3 OR $3 IS NULL )
            ORDER BY id DESC
            LIMIT $4
            "#,
            label as &ObjectLabel,
            namespace,
            token_id,
            max_page_size as i64
        )
        .fetch_all(&*self.pool)
        .await?;

        let next = (objects.len() == max_page_size)
            .then(|| {
                objects.last().map(|o| {
                    PaginateToken::V1(V1PaginateToken {
                        created_at: o.created_at,
                        id: o.id,
                    })
                    .to_string()
                })
            })
            .flatten();

        Ok((objects, next))
    }

    /// Add an association to the store.
    ///
    /// Associations are directed edges between objects.
    /// If an inverse association exists, it is automatically created.
    ///
    /// # Parameters
    /// - `from_id`: The identifier of the source object.
    /// - `label`: The label of the association.
    /// - `to_id`: The identifier of the target object.
    /// - `properties`: The properties of the association.
    ///
    /// # Returns
    /// The association that was added to the store.
    ///
    /// # Errors
    /// - [EntityNotFound](crate::Error::EntityNotFound): If the source or target object does not exist.
    /// - [AlreadyExists](crate::Error::AlreadyExists): If the association already exists.
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
    ///
    /// # Parameters
    /// - `from_id`: The identifier of the source object.
    /// - `label`: The label of the association.
    /// - `to_ids`: The identifiers of the target objects.
    ///
    /// # Returns
    /// The associations from the source object to the target objects.
    pub async fn get_associations(
        &self,
        from_id: &Uuid,
        label: &AssociationLabel,
        to_ids: &[Uuid],
        page_token: Option<&str>,
        max_page_size: Option<usize>,
    ) -> Result<(Vec<Association>, Option<String>)> {
        let max_page_size = usize::min(max_page_size.unwrap_or(MAX_PAGE_SIZE), MAX_PAGE_SIZE);
        let token = page_token
            .map(PaginateToken::<Uuid>::try_from)
            .transpose()?;
        let (_token_ts, token_id) = token
            .as_ref()
            .map(|PaginateToken::V1(V1PaginateToken { created_at, id })| (created_at, id))
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
              AND to_id = ANY($3)
              AND ( id < $4 OR $4 IS NULL )
            ORDER BY id DESC
            LIMIT $5
            "#,
            from_id,
            label as &AssociationLabel,
            &to_ids,
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
