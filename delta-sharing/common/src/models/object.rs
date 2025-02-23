use delta_sharing_derive::object_conversions;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::ResourceExt;
use crate::models::{
    CatalogInfo, SchemaInfo, ShareInfo, SharingSchemaInfo, SharingTable, StorageLocation, TableInfo,
};
use crate::{Error, ObjectLabel, Resource, ResourceIdent, ResourceName, ResourceRef};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct Object {
    /// The globally unique identifier of the object.
    pub id: Uuid,

    /// The label / type of the object.
    pub label: ObjectLabel,

    /// The namespaced name of the object.
    pub name: ResourceName,

    /// The properties of the object.
    pub properties: Option<serde_json::Value>,

    /// The time when the object was created.
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// The time when the object was last updated.
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl ResourceExt for Object {
    fn resource_label(&self) -> &ObjectLabel {
        &self.label
    }
    fn resource_name(&self) -> ResourceName {
        self.name.clone()
    }

    fn resource_ref(&self) -> ResourceRef {
        ResourceRef::Uuid(self.id)
    }
}

impl Object {
    pub fn resource_ident(&self) -> ResourceIdent {
        let id = ResourceRef::Uuid(self.id);
        self.label.to_ident(id)
    }
}

impl ResourceExt for Resource {
    fn resource_label(&self) -> &ObjectLabel {
        match self {
            Resource::ShareInfo(_) => &ObjectLabel::ShareInfo,
            Resource::SharingSchemaInfo(_) => &ObjectLabel::SharingSchemaInfo,
            Resource::SharingTable(_) => &ObjectLabel::SharingTable,
            Resource::Credential(_) => &ObjectLabel::Credential,
            Resource::StorageLocation(_) => &ObjectLabel::StorageLocation,
            Resource::CatalogInfo(_) => &ObjectLabel::CatalogInfo,
            Resource::SchemaInfo(_) => &ObjectLabel::SchemaInfo,
            Resource::TableInfo(_) => &ObjectLabel::TableInfo,
        }
    }

    fn resource_name(&self) -> ResourceName {
        match self {
            Resource::ShareInfo(share) => share.resource_name(),
            Resource::SharingSchemaInfo(schema) => schema.resource_name(),
            Resource::SharingTable(table) => table.resource_name(),
            Resource::Credential(_) => todo!(),
            Resource::StorageLocation(storage_location) => storage_location.resource_name(),
            Resource::CatalogInfo(catalog) => catalog.resource_name(),
            Resource::SchemaInfo(schema) => schema.resource_name(),
            Resource::TableInfo(table) => table.resource_name(),
        }
    }

    fn resource_ref(&self) -> ResourceRef {
        match self {
            Resource::ShareInfo(share) => share.resource_ref(),
            Resource::SharingSchemaInfo(schema) => schema.resource_ref(),
            Resource::SharingTable(table) => table.resource_ref(),
            Resource::Credential(_) => todo!(),
            Resource::StorageLocation(storage_location) => storage_location.resource_ref(),
            Resource::CatalogInfo(catalog) => catalog.resource_ref(),
            Resource::SchemaInfo(schema) => schema.resource_ref(),
            Resource::TableInfo(table) => table.resource_ref(),
        }
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
            ObjectLabel::SharingTable => Ok(Resource::SharingTable(object.try_into()?)),
            ObjectLabel::Credential => todo!("Convert Object to Resource"),
            ObjectLabel::StorageLocation => Ok(Resource::StorageLocation(object.try_into()?)),
            ObjectLabel::CatalogInfo => Ok(Resource::CatalogInfo(object.try_into()?)),
            ObjectLabel::SchemaInfo => Ok(Resource::SchemaInfo(object.try_into()?)),
            ObjectLabel::TableInfo => Ok(Resource::TableInfo(object.try_into()?)),
        }
    }
}

object_conversions!(
    StorageLocation, ObjectLabel::StorageLocation, id, [name];
    ShareInfo, ObjectLabel::ShareInfo, id, [name];
    SharingSchemaInfo, ObjectLabel::SharingSchemaInfo, id, [share, name];
    SharingTable, ObjectLabel::SharingTable, id, [share, schema, name], true;
    CatalogInfo, ObjectLabel::CatalogInfo, id, [name], true;
    SchemaInfo, ObjectLabel::SchemaInfo, schema_id, [catalog_name, name], true;
    TableInfo, ObjectLabel::TableInfo, table_id, [catalog_name, schema_name, name], true;
);
