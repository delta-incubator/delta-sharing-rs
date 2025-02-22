use crate::policy::{Permission, ResourceIdent};
use crate::{ResourceName, ResourceRef};

pub use super::catalog::v1::{
    CreateCatalogRequest, CreateSchemaRequest, DeleteCatalogRequest, DeleteSchemaRequest,
    GetCatalogRequest, GetSchemaRequest, ListCatalogsRequest, ListCatalogsResponse,
    ListSchemasRequest, ListSchemasResponse, UpdateCatalogRequest, UpdateSchemaRequest,
};
pub use super::credentials::v1::{
    CreateCredentialRequest, CreateStorageLocationRequest, DeleteCredentialRequest,
    DeleteStorageLocationRequest, GetCredentialRequest, GetStorageLocationRequest,
    ListStorageLocationsRequest,
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
        |req| ResourceIdent::share(ResourceName::new([&req.name])),
        Permission::Read
    ),
    (
        ListShareTablesRequest,
        |req| ResourceIdent::share(ResourceName::new([&req.name])),
        Permission::Read
    ),
    (
        ListSharingSchemasRequest,
        |req| ResourceIdent::share(ResourceName::new([&req.share])),
        Permission::Read
    ),
    (
        ListSchemaTablesRequest,
        |req| ResourceIdent::schema(ResourceName::new([&req.share, &req.name])),
        Permission::Read
    ),
    (
        GetTableVersionRequest,
        |req| ResourceIdent::sharing_table(ResourceName::new([&req.share, &req.schema, &req.name])),
        Permission::Read
    ),
    (
        GetTableMetadataRequest,
        |req| ResourceIdent::sharing_table(ResourceName::new([&req.share, &req.schema, &req.name])),
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
        |req| ResourceIdent::share(ResourceName::new([&req.name])),
        Permission::Manage
    ),
    (
        CreateSharingSchemaRequest,
        |req| ResourceIdent::share(ResourceName::new([&req.share])),
        Permission::Manage
    ),
    (
        DeleteSharingSchemaRequest,
        |req| ResourceIdent::schema(ResourceName::new([&req.share, &req.name])),
        Permission::Manage
    ),
    (
        CreateCredentialRequest,
        |_| ResourceIdent::Credential(ResourceRef::Undefined),
        Permission::Create
    ),
    (
        GetCredentialRequest,
        |req| ResourceIdent::credential(ResourceName::new([&req.name])),
        Permission::Read
    ),
    (
        DeleteCredentialRequest,
        |req| ResourceIdent::credential(ResourceName::new([&req.name])),
        Permission::Manage
    ),
    (
        CreateStorageLocationRequest,
        |_| ResourceIdent::StorageLocation(ResourceRef::Undefined),
        Permission::Create
    ),
    (
        GetStorageLocationRequest,
        |req| ResourceIdent::storage_location(ResourceName::new([&req.name])),
        Permission::Read
    ),
    (
        DeleteStorageLocationRequest,
        |req| ResourceIdent::storage_location(ResourceName::new([&req.name])),
        Permission::Manage
    ),
    (
        ListStorageLocationsRequest,
        |_| ResourceIdent::StorageLocation(ResourceRef::Undefined),
        Permission::Read
    ),
    (
        CreateCatalogRequest,
        |_| ResourceIdent::Catalog(ResourceRef::Undefined),
        Permission::Create
    ),
    (
        DeleteCatalogRequest,
        |req| ResourceIdent::catalog(ResourceName::new([&req.name])),
        Permission::Manage
    ),
    (
        UpdateCatalogRequest,
        |req| ResourceIdent::catalog(ResourceName::new([&req.name])),
        Permission::Manage
    ),
    (
        GetCatalogRequest,
        |req| ResourceIdent::catalog(ResourceName::new([&req.name])),
        Permission::Read
    ),
    (
        ListCatalogsRequest,
        |_| ResourceIdent::Catalog(ResourceRef::Undefined),
        Permission::Read
    ),
    (
        CreateSchemaRequest,
        |req| ResourceIdent::schema(ResourceName::new([&req.catalog_name, &req.name])),
        Permission::Create
    ),
    (
        DeleteSchemaRequest,
        |req| ResourceIdent::schema(ResourceName::from_naive_str_split(&req.full_name)),
        Permission::Manage
    ),
    (
        GetSchemaRequest,
        |req| ResourceIdent::schema(ResourceName::from_naive_str_split(&req.full_name)),
        Permission::Read
    ),
    (
        ListSchemasRequest,
        |req| ResourceIdent::catalog(ResourceName::new([&req.catalog_name])),
        Permission::Read
    ),
    (
        UpdateSchemaRequest,
        |req| ResourceIdent::schema(ResourceName::from_naive_str_split(&req.full_name)),
        Permission::Manage
    )
);
