use serde::Serialize;
use v1::GetShareRequest;

use crate::{
    policy::{AsResource, Permission, Resource, SecuredAction},
    ResourceRef,
};

#[allow(clippy::empty_docs, clippy::large_enum_variant)]
pub mod v1 {
    include!("gen/delta_sharing.v1.rs");
    #[cfg(feature = "grpc")]
    include!("gen/delta_sharing.v1.tonic.rs");
}

pub mod catalog {
    pub mod v1 {
        include!("gen/delta_sharing.catalog.v1.rs");
        #[cfg(feature = "grpc")]
        include!("gen/delta_sharing.catalog.v1.tonic.rs");
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub error_code: String,
    pub message: String,
}

impl AsResource for v1::Share {
    fn as_resource(&self) -> Resource {
        self.id
            .as_ref()
            .and_then(|id| {
                uuid::Uuid::parse_str(id)
                    .ok()
                    .map(|uuid| Resource::share(uuid))
            })
            .unwrap_or_else(|| Resource::share(&self.name))
    }
}

macro_rules! impl_secured_action {
    ($(($type:ty, $resource:expr, $permission:expr)),* $(,)?) => {
        $(
            impl SecuredAction for $type {
                fn resource(&self) -> Resource {
                    let f: fn(&$type) -> Resource = $resource;
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
        |req| Resource::share(&req.name),
        Permission::Read
    ),
    (
        v1::ListShareTablesRequest,
        |req| Resource::share(&req.name),
        Permission::Read
    ),
    (
        v1::ListSchemasRequest,
        |req| Resource::share(&req.share),
        Permission::Read
    ),
    (
        v1::ListSchemaTablesRequest,
        |req| Resource::schema(([&req.share], &req.name)),
        Permission::Read
    ),
    (
        v1::GetTableVersionRequest,
        |req| Resource::table(([&req.share, &req.schema], &req.name)),
        Permission::Read
    ),
    (
        v1::GetTableMetadataRequest,
        |req| Resource::table(([&req.share, &req.schema], &req.name)),
        Permission::Read
    ),
    (
        catalog::v1::CreateShareRequest,
        |_| Resource::Share(ResourceRef::Undefined),
        Permission::Create
    ),
    (
        catalog::v1::CreateSchemaRequest,
        |req| Resource::share(&req.share),
        Permission::Manage
    ),
);

macro_rules! impl_from_for_resource_ref {
    ($($ty:ty => $body:expr),+ $(,)?) => {
        $(
            impl From<$ty> for ResourceRef {
                fn from(req: $ty) -> Self {
                    ResourceRef::from($body(req))
                }
            }
        )+
    };
}

impl_from_for_resource_ref! {
    GetShareRequest => |req: GetShareRequest| req.name,
    v1::ListShareTablesRequest => |req: v1::ListShareTablesRequest| req.name,
    v1::ListSchemasRequest => |req: v1::ListSchemasRequest| req.share,
    v1::ListSchemaTablesRequest => |req: v1::ListSchemaTablesRequest| ([req.share], req.name),
    v1::GetTableVersionRequest => |req: v1::GetTableVersionRequest| ([req.share, req.schema], req.name),
    v1::GetTableMetadataRequest => |req: v1::GetTableMetadataRequest| ([req.share, req.schema], req.name),
}
