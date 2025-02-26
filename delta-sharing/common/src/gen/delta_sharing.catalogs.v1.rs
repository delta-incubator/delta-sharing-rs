// @generated
// This file is @generated by prost-build.
/// A catalog is a root-level namespace that contains schemas.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CatalogInfo {
    /// Unique identifier for the catalog.
    #[prost(string, optional, tag="1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    /// Name of catalog.
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// Username of current owner of catalog.
    #[prost(string, optional, tag="3")]
    pub owner: ::core::option::Option<::prost::alloc::string::String>,
    /// User-provided free-form text description.
    #[prost(string, optional, tag="4")]
    pub comment: ::core::option::Option<::prost::alloc::string::String>,
    /// A map of key-value properties attached to the securable.
    #[prost(message, optional, tag="5")]
    pub properties: ::core::option::Option<::pbjson_types::Struct>,
    /// Storage root URL for managed tables within catalog.
    #[prost(string, optional, tag="6")]
    pub storage_root: ::core::option::Option<::prost::alloc::string::String>,
    /// The name of delta sharing provider.
    ///
    /// A Delta Sharing catalog is a catalog that is based on a Delta share on a remote sharing server.
    #[prost(string, optional, tag="7")]
    pub provider_name: ::core::option::Option<::prost::alloc::string::String>,
    /// The name of the share under the share provider.
    #[prost(string, optional, tag="8")]
    pub share_name: ::core::option::Option<::prost::alloc::string::String>,
    /// The type of the catalog.
    #[prost(enumeration="CatalogType", optional, tag="9")]
    pub catalog_type: ::core::option::Option<i32>,
    /// Time at which this catalog was created, in epoch milliseconds.
    #[prost(int64, optional, tag="1000")]
    pub create_at: ::core::option::Option<i64>,
    /// Username of catalog creator.
    #[prost(string, optional, tag="1001")]
    pub created_by: ::core::option::Option<::prost::alloc::string::String>,
    /// Time at which this catalog was last updated, in epoch milliseconds.
    #[prost(int64, optional, tag="1002")]
    pub update_at: ::core::option::Option<i64>,
    /// Username of user who last modified catalog.
    #[prost(string, optional, tag="1003")]
    pub updated_by: ::core::option::Option<::prost::alloc::string::String>,
    /// Indicates whether the principal is limited to retrieving metadata
    /// for the associated object through the BROWSE privilege when include_browse
    /// is enabled in the request.
    #[prost(bool, optional, tag="1004")]
    pub browse_only: ::core::option::Option<bool>,
}
/// The type of the catalog.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CatalogType {
    /// Unknown catalog type.
    Unspecified = 0,
    ManagedCatalog = 1,
    DeltasharingCatalog = 2,
}
impl CatalogType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CatalogType::Unspecified => "CATALOG_TYPE_UNSPECIFIED",
            CatalogType::ManagedCatalog => "MANAGED_CATALOG",
            CatalogType::DeltasharingCatalog => "DELTASHARING_CATALOG",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CATALOG_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "MANAGED_CATALOG" => Some(Self::ManagedCatalog),
            "DELTASHARING_CATALOG" => Some(Self::DeltasharingCatalog),
            _ => None,
        }
    }
}
/// Create a new catalog
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCatalogRequest {
    /// Name of catalog.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// User-provided free-form text description.
    #[prost(string, optional, tag="2")]
    pub comment: ::core::option::Option<::prost::alloc::string::String>,
    /// A map of key-value properties attached to the securable.
    #[prost(message, optional, tag="3")]
    pub properties: ::core::option::Option<::pbjson_types::Struct>,
    /// Storage root URL for managed tables within catalog.
    #[prost(string, optional, tag="4")]
    pub storage_root: ::core::option::Option<::prost::alloc::string::String>,
    /// The name of delta sharing provider.
    ///
    /// A Delta Sharing catalog is a catalog that is based on a Delta share on a remote sharing server.
    #[prost(string, optional, tag="5")]
    pub provider_name: ::core::option::Option<::prost::alloc::string::String>,
    /// The name of the share under the share provider.
    #[prost(string, optional, tag="6")]
    pub share_name: ::core::option::Option<::prost::alloc::string::String>,
}
/// Delete a catalog
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCatalogRequest {
    /// Name of catalog.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Force deletion even if the catalog is not empty.
    #[prost(bool, optional, tag="2")]
    pub force: ::core::option::Option<bool>,
}
/// Get a catalog
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCatalogRequest {
    /// Name of catalog.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Whether to include catalogs in the response for which the principal can only access selective metadata for
    #[prost(bool, optional, tag="2")]
    pub include_browse: ::core::option::Option<bool>,
}
/// List catalogs
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCatalogsRequest {
    /// The maximum number of results per page that should be returned.
    #[prost(int32, optional, tag="2")]
    pub max_results: ::core::option::Option<i32>,
    /// Opaque pagination token to go to next page based on previous query.
    #[prost(string, optional, tag="3")]
    pub page_token: ::core::option::Option<::prost::alloc::string::String>,
}
/// List catalogs response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCatalogsResponse {
    /// The catalogs returned.
    #[prost(message, repeated, tag="1")]
    pub catalogs: ::prost::alloc::vec::Vec<CatalogInfo>,
    /// The next_page_token value to include in the next List request.
    #[prost(string, optional, tag="2")]
    pub next_page_token: ::core::option::Option<::prost::alloc::string::String>,
}
/// Update a catalog
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCatalogRequest {
    /// Name of catalog.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// User-provided free-form text description.
    #[prost(string, optional, tag="2")]
    pub comment: ::core::option::Option<::prost::alloc::string::String>,
    /// A map of key-value properties attached to the securable.
    #[prost(message, optional, tag="3")]
    pub properties: ::core::option::Option<::pbjson_types::Struct>,
    /// Name of catalog.
    #[prost(string, tag="4")]
    pub new_name: ::prost::alloc::string::String,
}
include!("delta_sharing.catalogs.v1.serde.rs");
// @@protoc_insertion_point(module)