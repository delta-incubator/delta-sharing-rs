use paste::paste;
use serde::Serialize;
use sharing::v1::{ShareInfo, SharingSchemaInfo};
use uuid::Uuid;

use crate::{
    policy::{AsResource, ResourceIdent},
    Error, ResourceRef,
};

pub use credentials::v1::{Credential, StorageLocation};
pub use properties::*;
pub use requests::*;

pub use internal::resource::{ObjectLabel, Resource};

mod properties;
pub(crate) mod requests;

pub use profiles::v1::Profile;
pub use sharing::v1::{Share, SharingSchema};

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

pub mod internal {
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
            .unwrap_or_else(|| ResourceIdent::share(&self.name))
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
impl_resource_conversions!(ShareInfo, SharingSchemaInfo, Credential, StorageLocation);

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
