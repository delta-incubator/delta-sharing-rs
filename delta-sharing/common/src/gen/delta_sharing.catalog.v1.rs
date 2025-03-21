// @generated
// This file is @generated by prost-build.
/// A catalog is a root-level namespace that contains schemas.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CatalogInfo {
    /// Name of catalog.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// User-provided free-form text description.
    #[prost(string, optional, tag="2")]
    pub comment: ::core::option::Option<::prost::alloc::string::String>,
    /// A map of key-value properties attached to the securable.
    #[prost(message, optional, tag="3")]
    pub properties: ::core::option::Option<::pbjson_types::Struct>,
    /// Username of current owner of catalog.
    #[prost(string, optional, tag="4")]
    pub owner: ::core::option::Option<::prost::alloc::string::String>,
    /// Time at which this catalog was created, in epoch milliseconds.
    #[prost(int64, optional, tag="5")]
    pub create_at: ::core::option::Option<i64>,
    /// Username of catalog creator.
    #[prost(string, optional, tag="6")]
    pub created_by: ::core::option::Option<::prost::alloc::string::String>,
    /// Time at which this catalog was last updated, in epoch milliseconds.
    #[prost(int64, optional, tag="7")]
    pub update_at: ::core::option::Option<i64>,
    /// Username of user who last modified catalog.
    #[prost(string, optional, tag="8")]
    pub updated_by: ::core::option::Option<::prost::alloc::string::String>,
    /// Unique identifier for the catalog.
    #[prost(string, optional, tag="9")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
}
/// A schema is a namespace within a catalog that contains tables.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SchemaInfo {
    /// Name of schema, relative to parent catalog.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Name of parent catalog.
    #[prost(string, tag="2")]
    pub catalog_name: ::prost::alloc::string::String,
    /// User-provided free-form text description.
    #[prost(string, optional, tag="3")]
    pub comment: ::core::option::Option<::prost::alloc::string::String>,
    /// A map of key-value properties attached to the securable.
    #[prost(message, optional, tag="4")]
    pub properties: ::core::option::Option<::pbjson_types::Struct>,
    /// Full name of schema, in form of catalog_name.schema_name.
    #[prost(string, optional, tag="5")]
    pub full_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Username of current owner of schema.
    #[prost(string, optional, tag="6")]
    pub owner: ::core::option::Option<::prost::alloc::string::String>,
    /// Time at which this schema was created, in epoch milliseconds.
    #[prost(int64, optional, tag="7")]
    pub create_at: ::core::option::Option<i64>,
    /// Username of schema creator.
    #[prost(string, optional, tag="8")]
    pub created_by: ::core::option::Option<::prost::alloc::string::String>,
    /// Time at which this schema was last updated, in epoch milliseconds.
    #[prost(int64, optional, tag="9")]
    pub update_at: ::core::option::Option<i64>,
    /// Username of user who last modified schema.
    #[prost(string, optional, tag="10")]
    pub updated_by: ::core::option::Option<::prost::alloc::string::String>,
    /// Unique identifier for the schema.
    #[prost(string, optional, tag="11")]
    pub schema_id: ::core::option::Option<::prost::alloc::string::String>,
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
}
/// List catalogs
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCatalogsRequest {
    /// The next_page_token value returned from a previous List request, if any.
    #[prost(string, optional, tag="1")]
    pub page_token: ::core::option::Option<::prost::alloc::string::String>,
    /// The maximum number of items to return.
    #[prost(int32, optional, tag="2")]
    pub max_results: ::core::option::Option<i32>,
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
/// Create a new Schema
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSchemaRequest {
    /// Name of schema, relative to parent catalog.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Name of parent catalog.
    #[prost(string, tag="2")]
    pub catalog_name: ::prost::alloc::string::String,
    /// User-provided free-form text description.
    #[prost(string, optional, tag="3")]
    pub comment: ::core::option::Option<::prost::alloc::string::String>,
    /// A map of key-value properties attached to the securable.
    #[prost(message, optional, tag="4")]
    pub properties: ::core::option::Option<::pbjson_types::Struct>,
}
/// Delete a Schema
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSchemaRequest {
    /// Full name of schema to delete.
    ///
    /// Format: catalog_name.schema_name
    #[prost(string, tag="1")]
    pub full_name: ::prost::alloc::string::String,
    /// Force deletion even if the schema is not empty.
    #[prost(bool, optional, tag="2")]
    pub force: ::core::option::Option<bool>,
}
/// Get a Schema
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSchemaRequest {
    /// Full name of schema.
    ///
    /// Format: catalog_name.schema_name
    #[prost(string, tag="1")]
    pub full_name: ::prost::alloc::string::String,
}
/// List Schemas in a catalog
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSchemasRequest {
    /// Name of parent catalog.
    #[prost(string, tag="1")]
    pub catalog_name: ::prost::alloc::string::String,
    /// The next_page_token value returned from a previous List request, if any.
    #[prost(string, optional, tag="2")]
    pub page_token: ::core::option::Option<::prost::alloc::string::String>,
    /// The maximum number of items to return.
    #[prost(int32, optional, tag="3")]
    pub max_results: ::core::option::Option<i32>,
}
/// List Schemas response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSchemasResponse {
    /// The schemas returned.
    #[prost(message, repeated, tag="1")]
    pub schemas: ::prost::alloc::vec::Vec<SchemaInfo>,
    /// The next_page_token value to include in the next List request.
    #[prost(string, optional, tag="2")]
    pub next_page_token: ::core::option::Option<::prost::alloc::string::String>,
}
/// Update a Schema
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSchemaRequest {
    /// Full name of schema, in form of catalog_name.schema_name.
    #[prost(string, tag="1")]
    pub full_name: ::prost::alloc::string::String,
    /// User-provided free-form text description.
    #[prost(string, optional, tag="2")]
    pub comment: ::core::option::Option<::prost::alloc::string::String>,
    /// A map of key-value properties attached to the securable.
    #[prost(message, optional, tag="3")]
    pub properties: ::core::option::Option<::pbjson_types::Struct>,
    /// Name of schema.
    #[prost(string, tag="4")]
    pub new_name: ::prost::alloc::string::String,
}
include!("delta_sharing.catalog.v1.serde.rs");
// @@protoc_insertion_point(module)