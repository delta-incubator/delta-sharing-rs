use catalog::v1::{SchemaInfo, ShareInfo};
use paste::paste;
use serde::Serialize;

use crate::{
    policy::{AsResource, ResourceIdent},
    Error, ResourceRef,
};

pub use catalog::v1::{Credential, StorageLocation};
pub use properties::*;
pub use requests::*;

pub use internal::resource::{ObjectLabel, Resource};

mod properties;
pub(crate) mod requests;

pub use v1::{Profile, Schema, Share};

#[allow(clippy::empty_docs, clippy::large_enum_variant)]
pub mod v1 {
    include!("../gen/delta_sharing.v1.rs");
    #[cfg(feature = "grpc")]
    include!("../gen/delta_sharing.v1.tonic.rs");
}

pub mod catalog {
    pub mod v1 {
        include!("../gen/delta_sharing.catalog.v1.rs");
        #[cfg(feature = "grpc")]
        include!("../gen/delta_sharing.catalog.v1.tonic.rs");
    }
}

mod internal {
    include!("../gen/delta_sharing.internal.rs");
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub error_code: String,
    pub message: String,
}

impl AsResource for v1::Share {
    fn as_resource(&self) -> ResourceIdent {
        self.id
            .as_ref()
            .and_then(|id| uuid::Uuid::parse_str(id).ok().map(ResourceIdent::share))
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
impl_resource_conversions!(ShareInfo, SchemaInfo, Credential, StorageLocation);

/// Conversions from more specific types to reduced info sharing API types
impl TryFrom<Resource> for v1::Share {
    type Error = Error;

    fn try_from(resource: Resource) -> Result<Self, Self::Error> {
        let info = ShareInfo::try_from(resource)?;
        Ok(v1::Share {
            id: Some(info.id),
            name: info.name,
        })
    }
}

impl TryFrom<Resource> for v1::Schema {
    type Error = Error;

    fn try_from(resource: Resource) -> Result<Self, Self::Error> {
        let info = SchemaInfo::try_from(resource)?;
        Ok(v1::Schema {
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
                ObjectLabel::SchemaInfo => assert_eq!(label.as_ref(), "schema_info"),
                ObjectLabel::Credential => assert_eq!(label.as_ref(), "credential"),
                ObjectLabel::StorageLocation => assert_eq!(label.as_ref(), "storage_location"),
            }
        }
    }
}
