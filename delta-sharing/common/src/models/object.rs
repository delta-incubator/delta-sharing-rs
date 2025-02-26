use delta_sharing_derive::object_conversions;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{ExternalLocationInfo, ResourceExt};
use crate::models::{
    CatalogInfo, Credential, SchemaInfo, ShareInfo, SharingSchemaInfo, SharingTable, TableInfo,
};
use crate::{Error, ObjectLabel, Resource, ResourceName, ResourceRef};

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

impl ResourceExt for Resource {
    fn resource_label(&self) -> &ObjectLabel {
        match self {
            Resource::ShareInfo(_) => &ObjectLabel::ShareInfo,
            Resource::SharingSchemaInfo(_) => &ObjectLabel::SharingSchemaInfo,
            Resource::SharingTable(_) => &ObjectLabel::SharingTable,
            Resource::Credential(_) => &ObjectLabel::Credential,
            Resource::CatalogInfo(_) => &ObjectLabel::CatalogInfo,
            Resource::SchemaInfo(_) => &ObjectLabel::SchemaInfo,
            Resource::TableInfo(_) => &ObjectLabel::TableInfo,
            Resource::ExternalLocationInfo(_) => &ObjectLabel::ExternalLocationInfo,
        }
    }

    fn resource_name(&self) -> ResourceName {
        match self {
            Resource::ShareInfo(obj) => obj.resource_name(),
            Resource::SharingSchemaInfo(obj) => obj.resource_name(),
            Resource::SharingTable(obj) => obj.resource_name(),
            Resource::Credential(_) => todo!(),
            Resource::CatalogInfo(obj) => obj.resource_name(),
            Resource::SchemaInfo(obj) => obj.resource_name(),
            Resource::TableInfo(obj) => obj.resource_name(),
            Resource::ExternalLocationInfo(obj) => obj.resource_name(),
        }
    }

    fn resource_ref(&self) -> ResourceRef {
        match self {
            Resource::ShareInfo(obj) => obj.resource_ref(),
            Resource::SharingSchemaInfo(obj) => obj.resource_ref(),
            Resource::SharingTable(obj) => obj.resource_ref(),
            Resource::Credential(_) => todo!(),
            Resource::CatalogInfo(obj) => obj.resource_ref(),
            Resource::SchemaInfo(obj) => obj.resource_ref(),
            Resource::TableInfo(obj) => obj.resource_ref(),
            Resource::ExternalLocationInfo(obj) => obj.resource_ref(),
        }
    }
}

impl TryFrom<Resource> for Object {
    type Error = Error;

    fn try_from(resource: Resource) -> Result<Self, Self::Error> {
        match resource {
            Resource::ShareInfo(obj) => obj.try_into(),
            Resource::SharingSchemaInfo(obj) => obj.try_into(),
            Resource::SharingTable(obj) => obj.try_into(),
            Resource::Credential(_) => Err(Error::generic("Cannot convert credential to object")),
            Resource::CatalogInfo(obj) => obj.try_into(),
            Resource::SchemaInfo(obj) => obj.try_into(),
            Resource::TableInfo(obj) => obj.try_into(),
            Resource::ExternalLocationInfo(obj) => obj.try_into(),
        }
    }
}

impl TryFrom<Object> for Resource {
    type Error = Error;

    fn try_from(obj: Object) -> Result<Self, Self::Error> {
        match obj.label {
            ObjectLabel::ShareInfo => Ok(Resource::ShareInfo(obj.try_into()?)),
            ObjectLabel::SharingSchemaInfo => Ok(Resource::SharingSchemaInfo(obj.try_into()?)),
            ObjectLabel::SharingTable => Ok(Resource::SharingTable(obj.try_into()?)),
            ObjectLabel::Credential => todo!("Convert Object to Resource"),
            ObjectLabel::CatalogInfo => Ok(Resource::CatalogInfo(obj.try_into()?)),
            ObjectLabel::SchemaInfo => Ok(Resource::SchemaInfo(obj.try_into()?)),
            ObjectLabel::TableInfo => Ok(Resource::TableInfo(obj.try_into()?)),
            ObjectLabel::ExternalLocationInfo => {
                Ok(Resource::ExternalLocationInfo(obj.try_into()?))
            }
        }
    }
}

object_conversions!(
    ExternalLocationInfo, ObjectLabel::ExternalLocationInfo, id, [name], true;
    ShareInfo, ObjectLabel::ShareInfo, id, [name];
    SharingSchemaInfo, ObjectLabel::SharingSchemaInfo, id, [share, name];
    SharingTable, ObjectLabel::SharingTable, id, [share, schema, name], true;
    CatalogInfo, ObjectLabel::CatalogInfo, id, [name], true;
    SchemaInfo, ObjectLabel::SchemaInfo, schema_id, [catalog_name, name], true;
    TableInfo, ObjectLabel::TableInfo, table_id, [catalog_name, schema_name, name], true;
    Credential, ObjectLabel::Credential, id, [name];
);
