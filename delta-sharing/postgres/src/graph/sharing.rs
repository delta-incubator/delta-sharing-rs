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
