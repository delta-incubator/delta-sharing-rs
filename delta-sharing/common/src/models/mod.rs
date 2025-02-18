use catalog::v1::{SchemaInfo, ShareInfo};
use paste::paste;
use serde::Serialize;

use crate::{
    policy::{AsResource, Permission, ResourceIdent, SecuredAction},
    Error, ResourceRef,
};

pub use catalog::v1::{resource::Resource, Credential, StorageLocation};
pub use conversions::*;

mod conversions;

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

macro_rules! impl_secured_action {
    ($(($type:ty, $resource:expr, $permission:expr)),* $(,)?) => {
        $(
            impl SecuredAction for $type {
                fn resource(&self) -> ResourceIdent {
                    let f: fn(&$type) -> ResourceIdent = $resource;
                    f(self)
                }
                fn permission(&self) -> Permission {
                    $permission
                }
            }
        )*
    };
}
impl_secured_action!(
    (
        v1::GetShareRequest,
        |req| ResourceIdent::share(&req.name),
        Permission::Read
    ),
    (
        v1::ListShareTablesRequest,
        |req| ResourceIdent::share(&req.name),
        Permission::Read
    ),
    (
        v1::ListSchemasRequest,
        |req| ResourceIdent::share(&req.share),
        Permission::Read
    ),
    (
        v1::ListSchemaTablesRequest,
        |req| ResourceIdent::schema(([&req.share], &req.name)),
        Permission::Read
    ),
    (
        v1::GetTableVersionRequest,
        |req| ResourceIdent::table(([&req.share, &req.schema], &req.name)),
        Permission::Read
    ),
    (
        v1::GetTableMetadataRequest,
        |req| ResourceIdent::table(([&req.share, &req.schema], &req.name)),
        Permission::Read
    ),
    (
        catalog::v1::CreateShareRequest,
        |_| ResourceIdent::Share(ResourceRef::Undefined),
        Permission::Create
    ),
    (
        catalog::v1::DeleteShareRequest,
        |_| ResourceIdent::share(ResourceRef::Undefined),
        Permission::Manage
    ),
    (
        catalog::v1::CreateSchemaRequest,
        |req| ResourceIdent::share(&req.share),
        Permission::Manage
    ),
    (
        catalog::v1::DeleteSchemaRequest,
        |req| ResourceIdent::share(&req.share),
        Permission::Manage
    ),
    (
        catalog::v1::CreateCredentialRequest,
        |_| ResourceIdent::Credential(ResourceRef::Undefined),
        Permission::Create
    ),
    (
        catalog::v1::DeleteCredentialRequest,
        |req| ResourceIdent::credential(&req.name),
        Permission::Manage
    ),
    (
        catalog::v1::CreateStorageLocationRequest,
        |_| ResourceIdent::StorageLocation(ResourceRef::Undefined),
        Permission::Create
    ),
    (
        catalog::v1::DeleteStorageLocationRequest,
        |req| ResourceIdent::storage_location(&req.name),
        Permission::Manage
    ),
    (
        catalog::v1::ListStorageLocationsRequest,
        |_| ResourceIdent::StorageLocation(ResourceRef::Undefined),
        Permission::Read
    ),
);

impl From<ResourceIdent> for ResourceRef {
    fn from(ident: ResourceIdent) -> Self {
        match ident {
            ResourceIdent::Share(r) => r,
            ResourceIdent::Schema(r) => r,
            ResourceIdent::Table(r) => r,
            ResourceIdent::Credential(r) => r,
            ResourceIdent::StorageLocation(r) => r,
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
            impl From<$type> for catalog::v1::resource::Resource {
                fn from(value: $type) -> Self {
                    paste!{ catalog::v1::resource::Resource::[<$type>](value) }
                }
            }

            impl TryFrom<catalog::v1::resource::Resource> for $type {
                type Error = Error;

                fn try_from(resource: catalog::v1::resource::Resource) -> Result<Self, Self::Error> {
                    match resource {
                        paste!{ catalog::v1::resource::Resource::[<$type>](value) } => Ok(value),
                        _ => Err(Error::generic(concat!("Resource is not a ", stringify!($type)))),
                    }
                }
            }
        )*
    };
}
impl_resource_conversions!(ShareInfo, SchemaInfo, Credential, StorageLocation);

/// Conversions from more specific types to reduced info sharing API types
impl TryFrom<catalog::v1::resource::Resource> for v1::Share {
    type Error = Error;

    fn try_from(resource: catalog::v1::resource::Resource) -> Result<Self, Self::Error> {
        let info = ShareInfo::try_from(resource)?;
        Ok(v1::Share {
            id: Some(info.id),
            name: info.name,
        })
    }
}

impl TryFrom<catalog::v1::resource::Resource> for v1::Schema {
    type Error = Error;

    fn try_from(resource: catalog::v1::resource::Resource) -> Result<Self, Self::Error> {
        let info = SchemaInfo::try_from(resource)?;
        Ok(v1::Schema {
            share: info.share,
            name: info.name,
            id: Some(info.id),
        })
    }
}
