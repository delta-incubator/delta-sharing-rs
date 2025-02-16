use std::collections::HashMap;

use delta_sharing_common::{
    Error, ResourceRef, Result, Schema, Share, SharingRepository, Table, TableLocationResover,
};

use super::GraphStore;
use crate::ObjectLabel;

#[async_trait::async_trait]
impl SharingRepository for GraphStore {
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

    async fn get_share(&self, id: &ResourceRef) -> Result<Share> {
        let object = match id {
            ResourceRef::Uuid(uuid) => self.get_object(uuid).await?,
            ResourceRef::Name(namespace, name) => {
                self.get_object_by_name(&ObjectLabel::Share, namespace, name)
                    .await?
            }
            ResourceRef::Undefined => return Err(Error::generic("Cannot get undefined share")),
        };
        Ok(Share {
            id: Some(object.id.hyphenated().to_string()),
            name: object.name,
        })
    }

    async fn delete_share(&self, id: &ResourceRef) -> Result<()> {
        match id {
            ResourceRef::Uuid(uuid) => Ok(self.delete_object(uuid).await?),
            ResourceRef::Name(namespace, name) => {
                let object = self
                    .get_object_by_name(&ObjectLabel::Share, namespace, name)
                    .await?;
                Ok(self.delete_object(&object.id).await?)
            }
            ResourceRef::Undefined => Ok(()),
        }
    }

    async fn list_shares(
        &self,
        max_results: Option<usize>,
        page_token: Option<String>,
    ) -> Result<(Vec<Share>, Option<String>)> {
        let objects = self
            .list_objects(&ObjectLabel::Share, &[], page_token.as_deref(), max_results)
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
        share: &ResourceRef,
        name: &str,
        comment: Option<String>,
        properties: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<Schema> {
        let mut properties = properties.unwrap_or_default();
        if let Some(comment) = comment {
            properties.insert("comment".to_string(), serde_json::Value::String(comment));
        }
        let json_map = serde_json::Map::from_iter(properties);
        let share = self.get_share(share).await?;
        let object = self
            .add_object(
                &ObjectLabel::Schema,
                &[share.name.clone()],
                name,
                Some(json_map.into()),
            )
            .await?;
        Ok(Schema {
            share: share.name,
            name: object.name,
            id: Some(object.id.hyphenated().to_string()),
        })
    }

    async fn get_schema(&self, id: &ResourceRef) -> Result<Schema> {
        let object = match id {
            ResourceRef::Uuid(uuid) => self.get_object(uuid).await?,
            ResourceRef::Name(namespace, name) => {
                self.get_object_by_name(&ObjectLabel::Schema, namespace, name)
                    .await?
            }
            ResourceRef::Undefined => return Err(Error::generic("Cannot get undefined schema")),
        };
        if object.namespace.len() != 1 {
            return Err(Error::generic(format!(
                "Schema with id {} has invalid namespace",
                object.id
            )));
        }
        Ok(Schema {
            share: object.namespace[0].clone(),
            name: object.name,
            id: Some(object.id.hyphenated().to_string()),
        })
    }

    async fn delete_schema(&self, id: &ResourceRef) -> Result<()> {
        match id {
            ResourceRef::Uuid(uuid) => Ok(self.delete_object(uuid).await?),
            ResourceRef::Name(namespace, name) => {
                let object = self
                    .get_object_by_name(&ObjectLabel::Schema, namespace, name)
                    .await?;
                Ok(self.delete_object(&object.id).await?)
            }
            ResourceRef::Undefined => Ok(()),
        }
    }

    async fn list_schemas(
        &self,
        share: &ResourceRef,
        max_results: Option<usize>,
        page_token: Option<String>,
    ) -> Result<(Vec<Schema>, Option<String>)> {
        let objects = self
            .list_objects(
                &ObjectLabel::Schema,
                &[share.to_string()],
                page_token.as_deref(),
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
                    id: Some(object.id.hyphenated().to_string()),
                })
                .collect(),
            objects.1,
        ))
    }

    async fn list_schema_tables(
        &self,
        schema: &ResourceRef,
        max_results: Option<usize>,
        page_token: Option<String>,
    ) -> Result<(Vec<Table>, Option<String>)> {
        todo!();
    }

    async fn list_share_tables(
        &self,
        share: &ResourceRef,
        max_results: Option<usize>,
        page_token: Option<String>,
    ) -> Result<(Vec<Table>, Option<String>)> {
        todo!();
    }
}

#[async_trait::async_trait]
impl TableLocationResover for GraphStore {
    async fn resolve(&self, table_ref: &ResourceRef) -> Result<url::Url> {
        todo!();
    }
}
