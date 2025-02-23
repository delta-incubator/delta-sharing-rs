use paste::paste;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    policy::{AsResource, ResourceIdent},
    Error, ResourceName, ResourceRef,
};

pub use credentials::v1::{Credential, StorageLocation};
pub use internal::resource::{ObjectLabel, Resource};
pub use object::*;
pub use properties::*;
pub use requests::*;
pub use tables::v1::TableInfo;

mod object;
mod properties;
pub(crate) mod requests;

pub use catalog::v1::{CatalogInfo, SchemaInfo};
pub use profiles::v1::Profile;
pub use sharing::v1::{Share, ShareInfo, SharingSchema, SharingSchemaInfo, SharingTable};

#[allow(clippy::empty_docs, clippy::large_enum_variant)]
pub mod sharing {
    pub mod v1 {
        include!("../gen/delta_sharing.sharing.v1.rs");
        #[cfg(feature = "grpc")]
        include!("../gen/delta_sharing.sharing.v1.tonic.rs");
    }
}

pub mod catalog {
    pub mod v1 {
        include!("../gen/delta_sharing.catalog.v1.rs");
        #[cfg(feature = "grpc")]
        include!("../gen/delta_sharing.catalog.v1.tonic.rs");
    }
}

pub mod credentials {
    pub mod v1 {
        include!("../gen/delta_sharing.credentials.v1.rs");
        #[cfg(feature = "grpc")]
        include!("../gen/delta_sharing.credentials.v1.tonic.rs");
    }
}

pub mod tables {
    pub mod v1 {
        include!("../gen/delta_sharing.tables.v1.rs");
        // #[cfg(feature = "grpc")]
        // include!("../gen/delta_sharing.tables.v1.tonic.rs");
    }
}

pub mod profiles {
    pub mod v1 {
        include!("../gen/delta_sharing.profiles.v1.rs");
        // #[cfg(feature = "grpc")]
        // include!("../gen/delta_sharing.profiles.v1.tonic.rs");
    }
}

pub(crate) mod internal {
    include!("../gen/delta_sharing.internal.rs");
}

impl ObjectLabel {
    pub fn to_ident(&self, id: impl Into<ResourceRef>) -> ResourceIdent {
        match self {
            ObjectLabel::ShareInfo => ResourceIdent::share(id),
            ObjectLabel::SharingSchemaInfo => ResourceIdent::schema(id),
            ObjectLabel::SharingTable => ResourceIdent::sharing_table(id),
            ObjectLabel::Credential => ResourceIdent::credential(id),
            ObjectLabel::StorageLocation => ResourceIdent::storage_location(id),
            ObjectLabel::CatalogInfo => ResourceIdent::catalog(id),
            ObjectLabel::SchemaInfo => ResourceIdent::schema(id),
            ObjectLabel::TableInfo => ResourceIdent::table(id),
        }
    }
}

pub trait ResourceExt {
    fn resource_label(&self) -> &ObjectLabel;
    fn resource_name(&self) -> ResourceName;
    fn resource_ref(&self) -> ResourceRef;
    fn resource_ident(&self) -> ResourceIdent {
        self.resource_label().to_ident(self.resource_ref())
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub error_code: String,
    pub message: String,
}

impl AsResource for Share {
    fn as_resource(&self) -> ResourceIdent {
        self.id
            .as_ref()
            .and_then(|id| Uuid::parse_str(id).ok().map(ResourceIdent::share))
            .unwrap_or_else(|| ResourceIdent::share(ResourceName::new([&self.name])))
    }
}

impl AsResource for ShareInfo {
    fn as_resource(&self) -> ResourceIdent {
        Uuid::parse_str(&self.id)
            .ok()
            .map(ResourceIdent::share)
            .unwrap_or_else(|| ResourceIdent::share(ResourceName::new([&self.name])))
    }
}

impl AsResource for SharingSchemaInfo {
    fn as_resource(&self) -> ResourceIdent {
        Uuid::parse_str(&self.id)
            .ok()
            .map(ResourceIdent::sharing_schema)
            .unwrap_or_else(|| {
                ResourceIdent::sharing_schema(ResourceName::new([&self.share, &self.name]))
            })
    }
}

impl AsResource for SharingSchema {
    fn as_resource(&self) -> ResourceIdent {
        self.id
            .as_ref()
            .and_then(|id| Uuid::parse_str(id).ok().map(ResourceIdent::sharing_schema))
            .unwrap_or_else(|| ResourceIdent::sharing_schema(ResourceName::new([&self.name])))
    }
}

impl AsResource for SharingTable {
    fn as_resource(&self) -> ResourceIdent {
        self.id
            .as_ref()
            .and_then(|id| Uuid::parse_str(id).ok().map(ResourceIdent::sharing_table))
            .unwrap_or_else(|| ResourceIdent::sharing_table(ResourceName::new([&self.name])))
    }
}

impl AsResource for Credential {
    fn as_resource(&self) -> ResourceIdent {
        Uuid::parse_str(&self.id)
            .ok()
            .map(ResourceIdent::credential)
            .unwrap_or_else(|| ResourceIdent::credential(ResourceName::new([&self.name])))
    }
}

impl AsResource for StorageLocation {
    fn as_resource(&self) -> ResourceIdent {
        Uuid::parse_str(&self.id)
            .ok()
            .map(ResourceIdent::storage_location)
            .unwrap_or_else(|| ResourceIdent::storage_location(ResourceName::new([&self.name])))
    }
}

impl AsResource for CatalogInfo {
    fn as_resource(&self) -> ResourceIdent {
        self.id
            .as_ref()
            .and_then(|id| Uuid::parse_str(id).ok().map(ResourceIdent::catalog))
            .unwrap_or_else(|| ResourceIdent::catalog(ResourceName::new([&self.name])))
    }
}

impl AsResource for SchemaInfo {
    fn as_resource(&self) -> ResourceIdent {
        self.schema_id
            .as_ref()
            .and_then(|id| Uuid::parse_str(id).ok().map(ResourceIdent::schema))
            .unwrap_or_else(|| {
                ResourceIdent::schema(ResourceName::new([&self.catalog_name, &self.name]))
            })
    }
}

impl AsResource for TableInfo {
    fn as_resource(&self) -> ResourceIdent {
        self.table_id
            .as_ref()
            .and_then(|id| Uuid::parse_str(id).ok().map(ResourceIdent::table))
            .unwrap_or_else(|| {
                ResourceIdent::table(ResourceName::new([
                    &self.catalog_name,
                    &self.schema_name,
                    &self.name,
                ]))
            })
    }
}

impl AsResource for Resource {
    fn as_resource(&self) -> ResourceIdent {
        match self {
            Resource::ShareInfo(share) => share.as_resource(),
            Resource::SharingSchemaInfo(schema) => schema.as_resource(),
            Resource::SharingTable(table) => table.as_resource(),
            Resource::Credential(cred) => cred.as_resource(),
            Resource::StorageLocation(loc) => loc.as_resource(),
            Resource::CatalogInfo(catalog) => catalog.as_resource(),
            Resource::SchemaInfo(schema) => schema.as_resource(),
            Resource::TableInfo(table) => table.as_resource(),
        }
    }
}

impl Resource {
    pub fn label(&self) -> &ObjectLabel {
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

    pub fn name(&self) -> ResourceName {
        match self {
            Resource::ShareInfo(info) => ResourceName::new([&info.name]),
            Resource::SharingSchemaInfo(info) => ResourceName::new([&info.share, &info.name]),
            Resource::SharingTable(info) => {
                ResourceName::new([&info.share, &info.schema, &info.name])
            }
            Resource::Credential(info) => ResourceName::new([&info.name]),
            Resource::StorageLocation(info) => ResourceName::new([&info.name]),
            Resource::CatalogInfo(info) => ResourceName::new([&info.name]),
            Resource::SchemaInfo(info) => ResourceName::new([&info.catalog_name, &info.name]),
            Resource::TableInfo(info) => {
                ResourceName::new([&info.catalog_name, &info.schema_name, &info.name])
            }
        }
    }
}

impl<T: SecuredAction> From<T> for ResourceRef {
    fn from(action: T) -> Self {
        action.resource().into()
    }
}

/// Convert a specific recorce type to and from the container type `Resource`
macro_rules! impl_resource_conversions {
    ($($type:ty),* $(,)?) => {
        $(
            impl From<$type> for Resource {
                fn from(value: $type) -> Self {
                    paste!{ Resource::[<$type>](value) }
                }
            }

            impl TryFrom<Resource> for $type {
                type Error = Error;

                fn try_from(resource: Resource) -> Result<Self, Self::Error> {
                    match resource {
                        paste!{ Resource::[<$type>](value) } => Ok(value),
                        _ => Err(Error::generic(concat!("Resource is not a ", stringify!($type)))),
                    }
                }
            }
        )*
    };
}
impl_resource_conversions!(
    ShareInfo,
    SharingSchemaInfo,
    Credential,
    StorageLocation,
    CatalogInfo,
    SchemaInfo
);

/// Conversions from more specific types to reduced info sharing API types
impl TryFrom<Resource> for Share {
    type Error = Error;

    fn try_from(resource: Resource) -> Result<Self, Self::Error> {
        let info = ShareInfo::try_from(resource)?;
        Ok(Share {
            id: Some(info.id),
            name: info.name,
        })
    }
}

impl TryFrom<Resource> for SharingSchema {
    type Error = Error;

    fn try_from(resource: Resource) -> Result<Self, Self::Error> {
        let info = SharingSchemaInfo::try_from(resource)?;
        Ok(SharingSchema {
            share: info.share,
            name: info.name,
            id: Some(info.id),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_object_label() {
        for label in ObjectLabel::iter() {
            match label {
                ObjectLabel::ShareInfo => assert_eq!(label.as_ref(), "share_info"),
                ObjectLabel::SharingSchemaInfo => assert_eq!(label.as_ref(), "sharing_schema_info"),
                ObjectLabel::Credential => assert_eq!(label.as_ref(), "credential"),
                ObjectLabel::StorageLocation => assert_eq!(label.as_ref(), "storage_location"),
                ObjectLabel::CatalogInfo => assert_eq!(label.as_ref(), "catalog_info"),
                ObjectLabel::SchemaInfo => assert_eq!(label.as_ref(), "schema_info"),
                ObjectLabel::TableInfo => assert_eq!(label.as_ref(), "table_info"),
                ObjectLabel::SharingTable => assert_eq!(label.as_ref(), "sharing_table"),
            }
        }
    }
}
