use crate::policy::{Permission, ResourceIdent};
use crate::ResourceRef;

pub use super::credentials::v1::{
    CreateCredentialRequest, CreateStorageLocationRequest, DeleteCredentialRequest,
    DeleteStorageLocationRequest, ListStorageLocationsRequest,
};
pub use super::sharing::v1::{
    CreateShareRequest, CreateSharingSchemaRequest, DeleteShareRequest, DeleteSharingSchemaRequest,
    GetShareRequest, GetTableMetadataRequest, GetTableVersionRequest, GetTableVersionResponse,
    ListSchemaTablesRequest, ListSchemaTablesResponse, ListShareTablesRequest,
    ListShareTablesResponse, ListSharesRequest, ListSharesResponse, ListSharingSchemasRequest,
    ListSharingSchemasResponse, QueryResponse,
};

pub trait SecuredAction: Send + Sync {
    /// The resource that the action is performed on.
    fn resource(&self) -> ResourceIdent;

    /// The permission required to perform the action.
    fn permission(&self) -> &'static Permission;
}

macro_rules! impl_secured_action {
    ($(($type:ty, $resource:expr, $permission:expr)),* $(,)?) => {
        $(
            impl SecuredAction for $type {
                fn resource(&self) -> ResourceIdent {
                    let f: fn(&$type) -> ResourceIdent = $resource;
                    f(self)
                }
                fn permission(&self) -> &'static Permission {
                    &$permission
                }
            }
        )*
    };
}

impl_secured_action!(
    (
        GetShareRequest,
        |req| ResourceIdent::share(&req.name),
        Permission::Read
    ),
    (
        ListShareTablesRequest,
        |req| ResourceIdent::share(&req.name),
        Permission::Read
    ),
    (
        ListSharingSchemasRequest,
        |req| ResourceIdent::share(&req.share),
        Permission::Read
    ),
    (
        ListSchemaTablesRequest,
        |req| ResourceIdent::schema(([&req.share], &req.name)),
        Permission::Read
    ),
    (
        GetTableVersionRequest,
        |req| ResourceIdent::table(([&req.share, &req.schema], &req.name)),
        Permission::Read
    ),
    (
        GetTableMetadataRequest,
        |req| ResourceIdent::table(([&req.share, &req.schema], &req.name)),
        Permission::Read
    ),
    (
        ListSharesRequest,
        |_| ResourceIdent::Share(ResourceRef::Undefined),
        Permission::Create
    ),
    (
        CreateShareRequest,
        |_| ResourceIdent::Share(ResourceRef::Undefined),
        Permission::Create
    ),
    (
        DeleteShareRequest,
        |req| ResourceIdent::share(&req.name),
        Permission::Manage
    ),
    (
        CreateSharingSchemaRequest,
        |req| ResourceIdent::share(&req.share),
        Permission::Manage
    ),
    (
        DeleteSharingSchemaRequest,
        |req| ResourceIdent::schema(([&req.share], &req.name)),
        Permission::Manage
    ),
    (
        CreateCredentialRequest,
        |_| ResourceIdent::Credential(ResourceRef::Undefined),
        Permission::Create
    ),
    (
        DeleteCredentialRequest,
        |req| ResourceIdent::credential(&req.name),
        Permission::Manage
    ),
    (
        CreateStorageLocationRequest,
        |_| ResourceIdent::StorageLocation(ResourceRef::Undefined),
        Permission::Create
    ),
    (
        DeleteStorageLocationRequest,
        |req| ResourceIdent::storage_location(&req.name),
        Permission::Manage
    ),
    (
        ListStorageLocationsRequest,
        |_| ResourceIdent::StorageLocation(ResourceRef::Undefined),
        Permission::Read
    ),
);
