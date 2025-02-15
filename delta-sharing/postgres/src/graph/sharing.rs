use std::collections::HashMap;

use delta_sharing_common::{Schema, Share};

use super::GraphStore;
use crate::{Error, ObjectLabel, Result, SchemaRef, ShareRef, SharingRepo};

#[async_trait::async_trait]
impl SharingRepo for GraphStore {
    async fn add_share(
        &self,
        name: &str,
        comment: Option<String>,
        properties: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<Share> {
        let mut properties = properties.unwrap_or_default();
        if let Some(comment) = comment {
            properties.insert("comment".to_string(), serde_json::Value::String(comment));
        }
        let json_map = serde_json::Map::from_iter(properties);
        let object = self
            .add_object(&ObjectLabel::Share, &[], name, Some(json_map.into()))
            .await?;
        Ok(Share {
            id: Some(object.id.hyphenated().to_string()),
            name: object.name,
        })
    }

    async fn get_share(&self, id: &ShareRef) -> Result<Share> {
        let object = match id {
            ShareRef::Uuid(uuid) => self.get_object(uuid).await?,
            ShareRef::Name(name) => {
                self.get_object_by_name(&ObjectLabel::Share, &[], name)
                    .await?
            }
        };
        Ok(Share {
            id: Some(object.id.hyphenated().to_string()),
            name: object.name,
        })
    }

    async fn delete_share(&self, id: &ShareRef) -> Result<()> {
        match id {
            ShareRef::Uuid(uuid) => self.delete_object(uuid).await,
            ShareRef::Name(name) => {
                let object = self
                    .get_object_by_name(&ObjectLabel::Share, &[], name)
                    .await?;
                self.delete_object(&object.id).await
            }
        }
    }

    async fn list_shares(
        &self,
        max_results: Option<usize>,
        page_token: Option<&str>,
    ) -> Result<(Vec<Share>, Option<String>)> {
        let objects = self
            .list_objects(&ObjectLabel::Share, &[], page_token, max_results)
            .await?;
        Ok((
            objects
                .0
                .into_iter()
                .map(|object| Share {
                    id: Some(object.id.hyphenated().to_string()),
                    name: object.name,
                })
                .collect(),
            objects.1,
        ))
    }

    async fn add_schema(
        &self,
        share: &str,
        name: &str,
        comment: Option<String>,
        properties: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<Schema> {
        let mut properties = properties.unwrap_or_default();
        if let Some(comment) = comment {
            properties.insert("comment".to_string(), serde_json::Value::String(comment));
        }
        let json_map = serde_json::Map::from_iter(properties);
        let object = self
            .add_object(
                &ObjectLabel::Schema,
                &[share.to_string()],
                name,
                Some(json_map.into()),
            )
            .await?;
        Ok(Schema {
            share: share.to_string(),
            name: object.name,
        })
    }

    async fn get_schema(&self, id: &SchemaRef) -> Result<Schema> {
        let object = match id {
            SchemaRef::Uuid(uuid) => self.get_object(uuid).await?,
            SchemaRef::Name((namespace, name)) => {
                self.get_object_by_name(&ObjectLabel::Schema, namespace, name)
                    .await?
            }
        };
        if object.namespace.len() != 1 {
            return Err(Error::EntityNotFound(format!(
                "Schema with id {} has invalid namespace",
                object.id
            )));
        }
        Ok(Schema {
            share: object.namespace[0].clone(),
            name: object.name,
        })
    }

    async fn delete_schema(&self, id: &SchemaRef) -> Result<()> {
        match id {
            SchemaRef::Uuid(uuid) => self.delete_object(uuid).await,
            SchemaRef::Name((namespace, name)) => {
                let object = self
                    .get_object_by_name(&ObjectLabel::Schema, namespace, name)
                    .await?;
                self.delete_object(&object.id).await
            }
        }
    }

    async fn list_schemas(
        &self,
        share: &str,
        max_results: Option<usize>,
        page_token: Option<&str>,
    ) -> Result<(Vec<Schema>, Option<String>)> {
        let objects = self
            .list_objects(
                &ObjectLabel::Schema,
                &[share.to_string()],
                page_token,
                max_results,
            )
            .await?;
        Ok((
            objects
                .0
                .into_iter()
                .map(|object| Schema {
                    share: share.to_string(),
                    name: object.name,
                })
                .collect(),
            objects.1,
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use uuid::Uuid;

    use super::*;
    use crate::Error;

    #[sqlx::test]
    async fn test_shares(pool: sqlx::PgPool) {
        let repo = super::GraphStore::new(Arc::new(pool));
        let share = repo
            .add_share("test_share", Some("test comment".to_string()), None)
            .await
            .unwrap();
        assert_eq!(share.name, "test_share");

        let share = repo
            .get_share(&ShareRef::Name("test_share".to_string()))
            .await
            .unwrap();
        assert_eq!(share.name, "test_share");

        let share = repo
            .get_share(&ShareRef::Uuid(share.id.unwrap().parse().unwrap()))
            .await
            .unwrap();
        assert_eq!(share.name, "test_share");

        // test get share with unknown id should fail
        let res = repo.get_share(&ShareRef::Uuid(Uuid::new_v4())).await;
        assert!(matches!(res, Err(Error::EntityNotFound(_))));

        // test get share with unknown name should fail
        let res = repo.get_share(&ShareRef::Name("unknown".to_string())).await;
        assert!(matches!(res, Err(Error::EntityNotFound(_))));

        repo.delete_share(&ShareRef::Name("test_share".to_string()))
            .await
            .unwrap();
        let res = repo
            .get_share(&ShareRef::Name("test_share".to_string()))
            .await;
        assert!(matches!(res, Err(Error::EntityNotFound(_))));

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
        let repo = super::GraphStore::new(Arc::new(pool));
        let share = repo
            .add_share("test_share", Some("test comment".to_string()), None)
            .await
            .unwrap();

        let schema = repo
            .add_schema(
                &share.name,
                "test_schema",
                Some("test comment".to_string()),
                None,
            )
            .await
            .unwrap();
        assert_eq!(schema.share, share.name);
        assert_eq!(schema.name, "test_schema");

        let schema = repo
            .get_schema(&SchemaRef::Name((
                vec![share.name.clone()],
                "test_schema".to_string(),
            )))
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
        let res = repo.get_schema(&SchemaRef::Uuid(Uuid::new_v4())).await;
        assert!(matches!(res, Err(Error::EntityNotFound(_))));

        // test get schema with unknown name should fail
        let res = repo
            .get_schema(&SchemaRef::Name((
                vec![share.name.clone()],
                "unknown".to_string(),
            )))
            .await;
        assert!(matches!(res, Err(Error::EntityNotFound(_))));

        repo.delete_schema(&SchemaRef::Name((
            vec![share.name.clone()],
            "test_schema".to_string(),
        )))
        .await
        .unwrap();
        let res = repo
            .get_schema(&SchemaRef::Name((
                vec![share.name.clone()],
                "test_schema".to_string(),
            )))
            .await;
        assert!(matches!(res, Err(Error::EntityNotFound(_))));

        // list schemas
        let schemas = repo.list_schemas(&share.name, None, None).await.unwrap();
        assert_eq!(schemas.0.len(), 0);

        let _schema = repo
            .add_schema(
                &share.name,
                "test_schema",
                Some("test comment".to_string()),
                None,
            )
            .await
            .unwrap();
        let schemas = repo.list_schemas(&share.name, None, None).await.unwrap();
        assert_eq!(schemas.0.len(), 1);
        assert_eq!(schemas.0[0].name, "test_schema");
    }
}
