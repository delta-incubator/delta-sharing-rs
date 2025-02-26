use serde::Serialize;

use crate::{Error, ResourceIdent, ResourceName, ResourceRef};

pub use credentials::v1::{Credential, StorageLocation};
pub use internal::resource::{ObjectLabel, Resource};
pub use object::*;
pub use properties::*;
pub use tables::v1::TableInfo;

mod object;
mod properties;

pub use catalogs::v1::CatalogInfo;
pub use profiles::v1::Profile;
pub use schemas::v1::SchemaInfo;
pub use sharing::v1::{Share, ShareInfo, SharingSchema, SharingSchemaInfo, SharingTable};

#[allow(clippy::empty_docs, clippy::large_enum_variant)]
pub mod sharing {
    pub mod v1 {
        include!("../gen/delta_sharing.sharing.v1.rs");
        #[cfg(feature = "grpc")]
        include!("../gen/delta_sharing.sharing.v1.tonic.rs");
    }
}

pub mod catalogs {
    pub mod v1 {
        include!("../gen/delta_sharing.catalogs.v1.rs");
        #[cfg(feature = "grpc")]
        include!("../gen/delta_sharing.catalogs.v1.tonic.rs");
    }
}

pub mod schemas {
    pub mod v1 {
        include!("../gen/delta_sharing.schemas.v1.rs");
        #[cfg(feature = "grpc")]
        include!("../gen/delta_sharing.schemas.v1.tonic.rs");
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
    /// Get the label for the resource
    fn resource_label(&self) -> &ObjectLabel;

    /// Get the name of the resource
    fn resource_name(&self) -> ResourceName;

    /// Get the reference for the resource
    ///
    /// Depending on the resource type, this may be a UUID or a name.
    /// If possible, implementations should prefer to use the UUID
    /// as it is globally unique. However not all repurce-like objects
    /// have a UUID field, or the UUID field may be optional.
    fn resource_ref(&self) -> ResourceRef;

    /// Get the ident for the resource
    fn resource_ident(&self) -> ResourceIdent {
        self.resource_label().to_ident(self.resource_ref())
    }
}

impl<T: ResourceExt> From<&T> for ResourceIdent {
    fn from(resource: &T) -> Self {
        resource.resource_ident()
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub error_code: String,
    pub message: String,
}

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
