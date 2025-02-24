//! Storage layer for managing objects and associations.
//!
//! Lossely based on the data model applied in Meta's [TAO].
//!
//! [TAO]: https://www.usenix.org/system/files/conference/atc13/atc13-bronson.pdf

use std::sync::Arc;

use delta_sharing_common::{ResourceIdent, ResourceRef};
use sqlx::migrate::Migrator;
use sqlx::PgPool;
use uuid::Uuid;

use super::{Association, AssociationLabel, Object, ObjectLabel};
use crate::constants::MAX_PAGE_SIZE;
use crate::pagination::V1PaginateToken;
use crate::resources::IdentRefs as _;
use crate::{error::Result, pagination::PaginateToken};

static MIGRATOR: Migrator = sqlx::migrate!();

#[derive(Debug, Clone)]
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

    /// Convert a resource reference to a UUID.
    ///
    /// If the reference is a name, the corresponding object is fetched from the store.
    /// to get the UUID. The object is returned as well in case it is needed later
    /// to avoid an additional fetch.
    ///
    /// # Parameters
    /// - `reference`: The reference to convert.
    ///
    /// # Returns
    /// The UUID of the reference and the object if the reference is a name.
    ///
    /// # Errors
    /// In case of an undefined reference, an error is returned.
    pub async fn ident_to_uuid(&self, reference: &ResourceIdent) -> Result<(Uuid, Option<Object>)> {
        let (label, ident) = reference.ident();
        match ident {
            ResourceRef::Uuid(id) => Ok((*id, None)),
            ResourceRef::Name(name) => {
                let object = self.get_object_by_name(label, name).await?;
                Ok((object.id, Some(object)))
            }
            ResourceRef::Undefined => Err(crate::Error::entity_not_found("undefined")),
        }
    }

    /// Add an object to the store.
    ///
    /// # Parameters
    /// - `label`: The label of the object.
    /// - `name`: The namespaced name of the object.
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
        name: &[String],
        properties: Option<serde_json::Value>,
    ) -> Result<Object> {
        Ok(sqlx::query_as!(
            Object,
            r#"
            INSERT INTO objects ( label, name, properties )
            VALUES ( $1, $2, $3 )
            RETURNING
                id,
                label AS "label: ObjectLabel",
                name,
                properties,
                created_at,
                updated_at
            "#,
            label as &ObjectLabel,
            name,
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
    pub async fn get_object_by_name(&self, label: &ObjectLabel, name: &[String]) -> Result<Object> {
        Ok(sqlx::query_as!(
            Object,
            r#"
            SELECT
                id,
                label AS "label: ObjectLabel",
                name,
                properties,
                created_at,
                updated_at
            FROM objects
            WHERE label = $1
              AND name = $2
            "#,
            label as &ObjectLabel,
            name
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
        new_label: impl Into<Option<&ObjectLabel>>,
        new_name: impl Into<Option<&[String]>>,
        properties: impl Into<Option<serde_json::Value>>,
    ) -> Result<Object> {
        Ok(sqlx::query_as!(
            Object,
            r#"
            UPDATE objects
            SET
                label = COALESCE($2, label),
                name = COALESCE($3, name),
                properties = COALESCE($4, properties)
            WHERE id = $1
            RETURNING
                id,
                label AS "label: ObjectLabel",
                name,
                properties,
                created_at,
                updated_at
            "#,
            id,
            new_label.into() as Option<&ObjectLabel>,
            new_name.into(),
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
                name,
                properties,
                created_at,
                updated_at
            FROM objects
            WHERE label = $1
                AND ( $2 = 0 OR name[1:$2] = $3)
                AND ( id < $4 OR $4 IS NULL )
            ORDER BY id DESC
            LIMIT $5
            "#,
            label as &ObjectLabel,
            namespace.len() as i32,
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
        let to_label = sqlx::query!(
            r#"
            SELECT id, label AS "label: ObjectLabel"
            FROM objects
            WHERE id = $1 OR id = $2
            "#,
            from_id,
            to_id
        )
        .fetch_all(&mut *txn)
        .await?;

        let id_map = to_label
            .into_iter()
            .map(|o| (o.id, o.label))
            .collect::<std::collections::HashMap<_, _>>();
        let to_label = id_map
            .get(to_id)
            .ok_or(crate::Error::entity_not_found("to_id"))?;
        let from_label = id_map
            .get(from_id)
            .ok_or(crate::Error::entity_not_found("from_id"))?;

        // Add the association.
        let association = sqlx::query_as!(
            Association,
            r#"
            INSERT INTO associations ( from_id, label, to_id, to_label, properties )
            VALUES ( $1, $2, $3, $4, $5 )
            RETURNING
                id,
                from_id,
                label AS "label: AssociationLabel",
                to_id,
                to_label as "to_label: ObjectLabel",
                properties,
                created_at,
                updated_at
            "#,
            from_id,
            label as &AssociationLabel,
            to_id,
            to_label as &ObjectLabel,
            properties.clone()
        )
        .fetch_one(&mut *txn)
        .await?;

        // Add the inverse association.
        if let Some(inverse_label) = label.inverse() {
            sqlx::query!(
                r#"
                INSERT INTO associations ( from_id, label, to_id, to_label, properties )
                VALUES ( $1, $2, $3, $4, $5 )
                "#,
                to_id,
                inverse_label as AssociationLabel,
                from_id,
                from_label as &ObjectLabel,
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
                updated_at,
                to_label as "to_label: ObjectLabel"
            FROM associations
            WHERE from_id = $1
              AND label = $2
              AND to_id = ANY($3)
              -- Pagination
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
        target_label: Option<&ObjectLabel>,
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
                updated_at,
                to_label as "to_label: ObjectLabel"
            FROM associations
            WHERE from_id = $1
              AND label = $2
              AND ( to_label = $3 OR $3 IS NULL )
              -- Pagination
              AND ( id < $4 OR $4 IS NULL )
            ORDER BY id DESC
            LIMIT $5
            "#,
            from_id,
            label as &AssociationLabel,
            target_label as Option<&ObjectLabel>,
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
