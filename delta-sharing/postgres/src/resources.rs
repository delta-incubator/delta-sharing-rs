use std::vec;

use delta_sharing_common::catalog::{self, StorageLocation};
use delta_sharing_common::models::{IntoJSONStruct, PropertyMap};
use delta_sharing_common::{
    AssociationLabel, Error, IntoJson, PropertyMapHandler, Resource, ResourceIdent, ResourceRef,
    ResourceStore, Result,
};
use itertools::Itertools;

use crate::{GraphStore, Object, ObjectLabel};

pub trait IdentRefs {
    fn ident(&self) -> (&ObjectLabel, &ResourceRef);
}

impl IdentRefs for ResourceIdent {
    fn ident(&self) -> (&ObjectLabel, &ResourceRef) {
        match self {
            ResourceIdent::Share(ident) => (&ObjectLabel::DeltaShare, ident),
            ResourceIdent::Schema(ident) => (&ObjectLabel::DeltaSchema, ident),
            ResourceIdent::Credential(ident) => (&ObjectLabel::Credential, ident),
            ResourceIdent::Table(ident) => (&ObjectLabel::Table, ident),
            ResourceIdent::StorageLocation(ident) => (&ObjectLabel::StorageLocation, ident),
        }
    }
}

fn extract_comment(properties: &Option<serde_json::Value>) -> Option<String> {
    properties
        .as_ref()
        .and_then(|properties| properties.get("comment"))
        .and_then(|comment| match comment {
            serde_json::Value::String(comment) => Some(comment.clone()),
            _ => None,
        })
}

impl TryFrom<catalog::ShareInfo> for Object {
    type Error = Error;

    fn try_from(share: catalog::ShareInfo) -> Result<Self, Self::Error> {
        Ok(Object {
            id: uuid::Uuid::parse_str(&share.id).unwrap_or_else(|_| uuid::Uuid::nil()),
            namespace: vec![],
            name: share.name,
            label: ObjectLabel::DeltaShare,
            properties: share
                .properties
                .map(PropertyMapHandler::proto_struct_to_json),
            updated_at: None,
            created_at: chrono::Utc::now(),
        })
    }
}

impl TryFrom<catalog::SchemaInfo> for Object {
    type Error = Error;

    fn try_from(schema: catalog::SchemaInfo) -> Result<Self, Self::Error> {
        Ok(Object {
            id: uuid::Uuid::parse_str(&schema.id).unwrap_or_else(|_| uuid::Uuid::nil()),
            namespace: vec![schema.share],
            name: schema.name,
            label: ObjectLabel::DeltaSchema,
            properties: schema
                .properties
                .map(PropertyMapHandler::proto_struct_to_json),
            updated_at: None,
            created_at: chrono::Utc::now(),
        })
    }
}

impl TryFrom<StorageLocation> for Object {
    type Error = Error;

    fn try_from(storage_location: StorageLocation) -> Result<Self, Self::Error> {
        let mut props = match storage_location.properties {
            Some(properties) => properties.into_json_struct(),
            None => serde_json::Map::new(),
        };
        props.insert("url".to_string(), storage_location.url.into());
        props.insert("credential".to_string(), storage_location.credential.into());
        props.insert("type".to_string(), storage_location.r#type.into());

        Ok(Object {
            id: uuid::Uuid::parse_str(&storage_location.id).unwrap_or_else(|_| uuid::Uuid::nil()),
            namespace: vec![],
            name: storage_location.name,
            label: ObjectLabel::StorageLocation,
            properties: Some(props.into()),
            updated_at: None,
            created_at: chrono::Utc::now(),
        })
    }
}

impl TryFrom<Resource> for Object {
    type Error = Error;

    fn try_from(resource: Resource) -> Result<Self, Self::Error> {
        match resource {
            Resource::ShareInfo(share) => share.try_into(),
            Resource::SchemaInfo(schema) => schema.try_into(),
            Resource::Credential(_) => Err(Error::generic("Cannot convert credential to object")),
            Resource::StorageLocation(storage_location) => storage_location.try_into(),
        }
    }
}

impl TryFrom<Object> for Resource {
    type Error = Error;

    fn try_from(object: Object) -> Result<Self, Self::Error> {
        match object.label {
            ObjectLabel::DeltaShare => Ok(Resource::ShareInfo(object.try_into()?)),
            ObjectLabel::DeltaSchema => Ok(Resource::SchemaInfo(object.try_into()?)),
            ObjectLabel::Credential => todo!("Convert Object to Resource"),
            ObjectLabel::Table => todo!("Convert Object to Resource"),
            ObjectLabel::StorageLocation => Ok(Resource::StorageLocation(object.try_into()?)),
            ObjectLabel::Principal => Err(Error::generic("Cannot convert principal to resource")),
        }
    }
}

impl TryFrom<Object> for StorageLocation {
    type Error = Error;

    fn try_from(object: Object) -> Result<Self, Self::Error> {
        let mut storage_location = StorageLocation {
            id: object.id.hyphenated().to_string(),
            name: object.name,
            ..Default::default()
        };
        match object.properties {
            Some(properties) => {
                let props = PropertyMapHandler::json_to_json_struct(properties)?;
                if let Some(serde_json::Value::String(url)) = props.get("url") {
                    storage_location.url = url.clone();
                }
                if let Some(serde_json::Value::String(credential)) = props.get("credential") {
                    storage_location.credential = credential.clone();
                }
                if let Some(serde_json::Value::Number(typ)) = props.get("type") {
                    storage_location.r#type = typ
                        .as_i64()
                        .ok_or_else(|| Error::generic("expected integer"))?
                        as i32;
                }
            }
            None => return Err(Error::generic("Storage location must have properties")),
        };
        Ok(storage_location)
    }
}

impl TryFrom<Object> for catalog::ShareInfo {
    type Error = Error;

    fn try_from(object: Object) -> Result<Self, Self::Error> {
        Ok(catalog::ShareInfo {
            id: object.id.hyphenated().to_string(),
            name: object.name,
            description: extract_comment(&object.properties),
            properties: object
                .properties
                .map(PropertyMapHandler::json_to_proto_struct)
                .transpose()?,
        })
    }
}

impl TryFrom<Object> for catalog::SchemaInfo {
    type Error = Error;

    fn try_from(object: Object) -> Result<Self, Self::Error> {
        Ok(catalog::SchemaInfo {
            id: object.id.hyphenated().to_string(),
            share_id: None,
            name: object.name,
            share: object
                .namespace
                .last()
                .cloned()
                .ok_or_else(|| Error::generic("Schema must have a share as a parent resource"))?,
            description: extract_comment(&object.properties),
            properties: object
                .properties
                .map(PropertyMapHandler::json_to_proto_struct)
                .transpose()?,
        })
    }
}

#[async_trait::async_trait]
impl ResourceStore for GraphStore {
    /// Get a resource by its identifier.
    ///
    /// # Arguments
    /// - `id`: The identifier of the resource to get.
    ///
    /// # Returns
    /// The resource with the given identifier.
    async fn get(&self, id: &ResourceIdent) -> Result<(Resource, ResourceRef)> {
        let (label, ident) = id.ident();
        match ident {
            ResourceRef::Uuid(uuid) => {
                Ok((self.get_object(uuid).await?.try_into()?, ident.clone()))
            }
            ResourceRef::Name(namespace, name) => {
                let object = self.get_object_by_name(label, namespace, name).await?;
                let id_new = ResourceRef::Uuid(object.id.clone());
                Ok((object.try_into()?, id_new))
            }
            ResourceRef::Undefined => Err(Error::generic("Cannot get undefined resource")),
        }
    }

    /// List resources.
    ///
    /// List resources in the store that are children of the given resource.
    /// If the Reference inside the ResourceIdent is [Undefined](crate::ResourceRef::Undefined),
    /// the root of the store is used and resources of the specified type are listed.
    ///
    /// # Arguments
    /// - `root`: The root resource to list children of.
    /// - `max_results`: The maximum number of results to return.
    /// - `page_token`: The token to use to get the next page of results.
    async fn list(
        &self,
        root: &ResourceIdent,
        max_results: Option<usize>,
        page_token: Option<String>,
    ) -> Result<(Vec<Resource>, Option<String>)> {
        let (label, ident) = root.ident();
        match ident {
            ResourceRef::Undefined => {
                let objects = self
                    .list_objects(label, &[], page_token.as_deref(), max_results)
                    .await?;
                Ok((
                    objects
                        .0
                        .into_iter()
                        .map(|object| object.try_into())
                        .try_collect()?,
                    objects.1,
                ))
            }
            _ => Err(Error::generic("Cannot list children of non-root resource")),
        }
    }

    /// Create a new resource.
    ///
    /// # Arguments
    /// - `resource`: The resource to create.
    ///
    /// # Returns
    /// The created resource.
    async fn create(&self, resource: Resource) -> Result<(Resource, ResourceRef)> {
        let object: Object = resource.try_into()?;
        let object = self
            .add_object(
                &object.label,
                &object.namespace,
                &object.name,
                object.properties,
            )
            .await?;
        let id = ResourceRef::Uuid(object.id.clone());
        Ok((object.try_into()?, id))
    }

    /// Delete a resource by its identifier.
    ///
    /// # Arguments
    /// - `id`: The identifier of the resource to delete.
    async fn delete(&self, id: &ResourceIdent) -> Result<()> {
        let (label, ident) = id.ident();
        match ident {
            ResourceRef::Uuid(uuid) => self.delete_object(uuid).await?,
            ResourceRef::Name(namespace, name) => {
                let obj = self.get_object_by_name(label, namespace, name).await?;
                self.delete_object(&obj.id).await?;
            }
            ResourceRef::Undefined => {
                return Err(Error::generic("Cannot delete undefined resource"))
            }
        };
        Ok(())
    }

    /// Add an association between two resources.
    async fn add_association(
        &self,
        from: &ResourceIdent,
        to: &ResourceIdent,
        label: &AssociationLabel,
        properties: Option<PropertyMap>,
    ) -> Result<()> {
        let (from_label, from_ident) = from.ident();
        let (to_label, to_ident) = to.ident();
        let from_id = match from_ident {
            ResourceRef::Uuid(uuid) => *uuid,
            ResourceRef::Name(namespace, name) => {
                let object = self.get_object_by_name(from_label, namespace, name).await?;
                object.id
            }
            ResourceRef::Undefined => {
                return Err(Error::generic(
                    "Cannot add association to undefined resource",
                ))
            }
        };
        let to_id = match to_ident {
            ResourceRef::Uuid(uuid) => *uuid,
            ResourceRef::Name(namespace, name) => {
                let object = self.get_object_by_name(to_label, namespace, name).await?;
                object.id
            }
            ResourceRef::Undefined => {
                return Err(Error::generic(
                    "Cannot add association to undefined resource",
                ))
            }
        };
        self.add_association(&from_id, label, &to_id, properties.map(|p| p.into_json()))
            .await?;
        Ok(())
    }

    async fn remove_association(
        &self,
        from: &ResourceIdent,
        to: &ResourceIdent,
        label: &AssociationLabel,
    ) -> Result<()> {
        todo!("remove_association")
    }

    async fn list_associations(
        &self,
        resource: &ResourceIdent,
        label: &AssociationLabel,
        target_label: Option<&ResourceIdent>,
        max_results: Option<usize>,
        page_token: Option<String>,
    ) -> Result<(Vec<ResourceIdent>, Option<String>)> {
        let target_label = target_label.map(|r| r.ident().0);
        let target_id = self.ident_to_uuid(resource).await?;
        let (associations, token) = self
            .list_associations(
                &target_id,
                label,
                target_label,
                page_token.as_deref(),
                max_results,
            )
            .await?;
        let idents = associations
            .into_iter()
            .map(|assoc| assoc.to_label.to_ident(&assoc.to_id))
            .collect();
        Ok((idents, token))
    }
}
