use serde::Serialize;

use crate::policy::{AsResource, Permission, Resource, SecuredAction};

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

pub struct TableRef {
    pub share: String,
    pub schema: String,
    pub table: String,
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
        catalog::v1::CreateSchemaRequest,
        |req| Resource::share(&req.share),
        Permission::Manage
    ),
);
