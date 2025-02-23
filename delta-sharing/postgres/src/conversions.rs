use delta_sharing_common::models::{
    SchemaInfo, SharingSchemaInfo, SharingTable, StorageLocation, TableInfo,
};
use delta_sharing_common::{
    CatalogInfo, Error, ObjectLabel, Resource, ResourceName, Result, ShareInfo,
};
use delta_sharing_derive::object_conversions;

use crate::Object;

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
