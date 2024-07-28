// @generated
/// / Register a new table in the Delta Sharing service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTableRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub location: ::prost::alloc::string::String,
}
/// / Response to CreateTableRequest.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTableResponse {
    #[prost(string, tag="1")]
    pub table_uri: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSchemaRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
include!("delta_sharing.catalog.v1.serde.rs");
// @@protoc_insertion_point(module)