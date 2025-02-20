use std::vec;
use uuid::Uuid;

use delta_sharing_common::models::catalog::v1 as catalog;
use delta_sharing_common::models::credentials::v1 as credentials;
use delta_sharing_common::models::internal::resource::ObjectLabel;
use delta_sharing_common::models::sharing::v1 as sharing;
use delta_sharing_common::models::tables::v1 as tables;
use delta_sharing_common::models::{IntoJSONStruct, PropertyMap};
use delta_sharing_common::{
    AssociationLabel, Error, IntoJson, PropertyMapHandler, Resource, ResourceIdent, ResourceRef,
    ResourceStore, Result,
};
use itertools::Itertools;

use crate::{GraphStore, Object};

pub trait IdentRefs {
    fn ident(&self) -> (&ObjectLabel, &ResourceRef);
}

impl IdentRefs for ResourceIdent {
    fn ident(&self) -> (&ObjectLabel, &ResourceRef) {
        match self {
            ResourceIdent::Share(ident) => (&ObjectLabel::ShareInfo, ident),
            ResourceIdent::SharingSchema(ident) => (&ObjectLabel::SharingSchemaInfo, ident),
            ResourceIdent::SharingTable(ident) => (&ObjectLabel::SharingTable, ident),
            ResourceIdent::Credential(ident) => (&ObjectLabel::Credential, ident),
            ResourceIdent::StorageLocation(ident) => (&ObjectLabel::StorageLocation, ident),
            ResourceIdent::Catalog(ident) => (&ObjectLabel::CatalogInfo, ident),
            ResourceIdent::Schema(ident) => (&ObjectLabel::SchemaInfo, ident),
            ResourceIdent::Table(ident) => (&ObjectLabel::TableInfo, ident),
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

impl TryFrom<sharing::ShareInfo> for Object {
    type Error = Error;

    fn try_from(share: sharing::ShareInfo) -> Result<Self, Self::Error> {
        Ok(Object {
            id: Uuid::parse_str(&share.id).unwrap_or_else(|_| Uuid::nil()),
            namespace: vec![],
            name: share.name,
            label: ObjectLabel::ShareInfo,
            properties: share
                .properties
                .map(PropertyMapHandler::proto_struct_to_json),
            updated_at: None,
            created_at: chrono::Utc::now(),
        })
    }
}

impl TryFrom<sharing::SharingSchemaInfo> for Object {
    type Error = Error;

    fn try_from(schema: sharing::SharingSchemaInfo) -> Result<Self, Self::Error> {
        Ok(Object {
            id: Uuid::parse_str(&schema.id).unwrap_or_else(|_| Uuid::nil()),
            namespace: vec![schema.share],
            name: schema.name,
            label: ObjectLabel::SharingSchemaInfo,
            properties: schema
                .properties
                .map(PropertyMapHandler::proto_struct_to_json),
            updated_at: None,
            created_at: chrono::Utc::now(),
        })
    }
}

impl TryFrom<sharing::SharingTable> for Object {
    type Error = Error;

    fn try_from(table: sharing::SharingTable) -> Result<Self, Self::Error> {
        Ok(Object {
            id: table
                .id
                .and_then(|id| Uuid::parse_str(&id).ok())
                .unwrap_or_else(Uuid::nil),
            namespace: vec![table.share, table.schema],
            name: table.name,
            label: ObjectLabel::SharingTable,
            properties: None,
            updated_at: None,
            created_at: chrono::Utc::now(),
        })
    }
}

impl TryFrom<credentials::StorageLocation> for Object {
    type Error = Error;

    fn try_from(storage_location: credentials::StorageLocation) -> Result<Self, Self::Error> {
        let mut props = match storage_location.properties {
            Some(properties) => properties.into_json_struct(),
            None => serde_json::Map::new(),
        };
        props.insert("url".to_string(), storage_location.url.into());
        props.insert("credential".to_string(), storage_location.credential.into());
        props.insert("type".to_string(), storage_location.r#type.into());

        Ok(Object {
            id: Uuid::parse_str(&storage_location.id).unwrap_or_else(|_| Uuid::nil()),
            namespace: vec![],
            name: storage_location.name,
            label: ObjectLabel::StorageLocation,
            properties: Some(props.into()),
            updated_at: None,
            created_at: chrono::Utc::now(),
        })
    }
}

impl TryFrom<catalog::CatalogInfo> for Object {
    type Error = Error;

    fn try_from(catalog: catalog::CatalogInfo) -> Result<Self, Self::Error> {
        Ok(Object {
            id: catalog
                .id
                .and_then(|id| Uuid::parse_str(&id).ok())
                .unwrap_or_else(Uuid::nil),
            namespace: vec![],
            name: catalog.name,
            label: ObjectLabel::CatalogInfo,
            properties: catalog
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
            id: schema
                .schema_id
                .and_then(|id| Uuid::parse_str(&id).ok())
                .unwrap_or_else(Uuid::nil),
            namespace: vec![schema.catalog_name],
            name: schema.name,
            label: ObjectLabel::SchemaInfo,
            properties: schema
                .properties
                .map(PropertyMapHandler::proto_struct_to_json),
            updated_at: None,
            created_at: chrono::Utc::now(),
        })
    }
}

impl TryFrom<tables::TableInfo> for Object {
    type Error = Error;

    fn try_from(table: tables::TableInfo) -> Result<Self, Self::Error> {
        Ok(Object {
            id: table
                .table_id
                .and_then(|id| Uuid::parse_str(&id).ok())
                .unwrap_or_else(Uuid::nil),
            namespace: vec![table.catalog_name, table.schema_name],
            name: table.name,
            label: ObjectLabel::TableInfo,
            properties: table
                .properties
                .map(PropertyMapHandler::proto_struct_to_json),
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
            Resource::SharingSchemaInfo(schema) => schema.try_into(),
            Resource::SharingTable(table) => table.try_into(),
            Resource::Credential(_) => Err(Error::generic("Cannot convert credential to object")),
            Resource::StorageLocation(storage_location) => storage_location.try_into(),
            Resource::CatalogInfo(catalog) => catalog.try_into(),
            Resource::SchemaInfo(schema) => schema.try_into(),
            Resource::TableInfo(table) => table.try_into(),
        }
    }
}

impl TryFrom<Object> for Resource {
    type Error = Error;

    fn try_from(object: Object) -> Result<Self, Self::Error> {
        match object.label {
            ObjectLabel::ShareInfo => Ok(Resource::ShareInfo(object.try_into()?)),
            ObjectLabel::SharingSchemaInfo => Ok(Resource::SharingSchemaInfo(object.try_into()?)),
            ObjectLabel::SharingTable => todo!("Convert Object to Resource"),
            ObjectLabel::Credential => todo!("Convert Object to Resource"),
            ObjectLabel::StorageLocation => Ok(Resource::StorageLocation(object.try_into()?)),
            ObjectLabel::CatalogInfo => Ok(Resource::CatalogInfo(object.try_into()?)),
            ObjectLabel::SchemaInfo => Ok(Resource::SchemaInfo(object.try_into()?)),
            ObjectLabel::TableInfo => Ok(Resource::TableInfo(object.try_into()?)),
        }
    }
}

impl TryFrom<Object> for credentials::StorageLocation {
    type Error = Error;

    fn try_from(object: Object) -> Result<Self, Self::Error> {
        let mut storage_location = credentials::StorageLocation {
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

impl TryFrom<Object> for sharing::ShareInfo {
    type Error = Error;

    fn try_from(object: Object) -> Result<Self, Self::Error> {
        Ok(sharing::ShareInfo {
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

impl TryFrom<Object> for sharing::SharingSchemaInfo {
    type Error = Error;

    fn try_from(object: Object) -> Result<Self, Self::Error> {
        Ok(sharing::SharingSchemaInfo {
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

impl TryFrom<Object> for sharing::SharingTable {
    type Error = Error;

    fn try_from(object: Object) -> Result<Self, Self::Error> {
        let schema = object
            .namespace
            .get(1)
            .cloned()
            .ok_or_else(|| Error::generic("Table must have a schema as a parent resource"))?;
        let share = object
            .namespace
            .first()
            .cloned()
            .ok_or_else(|| Error::generic("Table must have a share as a parent resource"))?;
        Ok(sharing::SharingTable {
            id: Some(object.id.hyphenated().to_string()),
            name: object.name,
            share,
            schema,
            share_id: None,
        })
    }
}

impl TryFrom<Object> for catalog::CatalogInfo {
    type Error = Error;

    fn try_from(object: Object) -> Result<Self, Self::Error> {
        Ok(catalog::CatalogInfo {
            id: Some(object.id.hyphenated().to_string()),
            name: object.name,
            comment: extract_comment(&object.properties),
            properties: object
                .properties
                .map(PropertyMapHandler::json_to_proto_struct)
                .transpose()?,
            owner: None,
            created_by: None,
            updated_by: None,
            create_at: None,
            update_at: None,
        })
    }
}

impl TryFrom<Object> for catalog::SchemaInfo {
    type Error = Error;

    fn try_from(object: Object) -> Result<Self, Self::Error> {
        Ok(catalog::SchemaInfo {
            schema_id: Some(object.id.hyphenated().to_string()),
            catalog_name: object.namespace.first().cloned().unwrap_or_default(),
            name: object.name,
            comment: extract_comment(&object.properties),
            properties: object
                .properties
                .map(PropertyMapHandler::json_to_proto_struct)
                .transpose()?,
            full_name: None,
            owner: None,
            created_by: None,
            updated_by: None,
            create_at: None,
            update_at: None,
        })
    }
}

impl TryFrom<Object> for tables::TableInfo {
    type Error = Error;

    fn try_from(object: Object) -> Result<Self, Self::Error> {
        Ok(tables::TableInfo {
            table_id: Some(object.id.hyphenated().to_string()),
            catalog_name: object.namespace.first().cloned().unwrap_or_default(),
            schema_name: object.namespace.get(1).cloned().unwrap_or_default(),
            name: object.name,
            comment: extract_comment(&object.properties),
            properties: object
                .properties
                .map(PropertyMapHandler::json_to_proto_struct)
                .transpose()?,
            table_type: 0,
            data_source_format: 0,
            full_name: None,
            owner: None,
            created_by: None,
            updated_by: None,
            create_at: None,
            update_at: None,
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
                let id_new = ResourceRef::Uuid(object.id);
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
        let id = ResourceRef::Uuid(object.id);
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
            .map(|assoc| assoc.to_label.to_ident(assoc.to_id))
            .collect();
        Ok((idents, token))
    }
}
