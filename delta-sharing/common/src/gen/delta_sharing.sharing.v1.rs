// @generated
// This file is @generated by prost-build.
/// A share is a logical grouping to share with recipients. A share can be shared with one or multiple recipients.
/// A recipient can access all resources in a share. A share may contain multiple schemas.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Share {
    /// Name of the share.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Unique identifier for the share.
    #[prost(string, optional, tag="2")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
}
/// A schema is a logical grouping of tables. A schema may contain multiple tables.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SharingSchema {
    /// The name of the schama
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The share name that the schema belongs to.
    #[prost(string, tag="2")]
    pub share: ::prost::alloc::string::String,
    /// Unique identifier for the schema.
    #[prost(string, optional, tag="3")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
}
/// A table is a Delta Lake table or a view on top of a Delta Lake table.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SharingTable {
    /// The name of the table.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The schema name that the table belongs to.
    #[prost(string, tag="2")]
    pub schema: ::prost::alloc::string::String,
    /// The share name that the table belongs to.
    #[prost(string, tag="3")]
    pub share: ::prost::alloc::string::String,
    /// Unique identifier for the table.
    #[prost(string, optional, tag="4")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    /// A unique identifier for the share this table belongs to.
    #[prost(string, optional, tag="5")]
    pub share_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShareInfo {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, optional, tag="3")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="4")]
    pub properties: ::core::option::Option<::pbjson_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SharingSchemaInfo {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub share: ::prost::alloc::string::String,
    #[prost(string, optional, tag="4")]
    pub share_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="6")]
    pub properties: ::core::option::Option<::pbjson_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Format {
    /// Name of the encoding for files in this table
    #[prost(string, tag="1")]
    pub provider: ::prost::alloc::string::String,
    /// A map containing configuration options for the format
    #[prost(map="string, string", tag="2")]
    pub options: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metadata {
    /// Unique identifier for this table
    /// Validate GUID
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// User-provided identifier for this table
    #[prost(string, optional, tag="2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// User-provided description for this table
    #[prost(string, optional, tag="3")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    /// Specification of the encoding for the files stored in the table
    #[prost(message, optional, tag="4")]
    pub format: ::core::option::Option<Format>,
    /// Schema of the table
    #[prost(string, tag="5")]
    pub schema_string: ::prost::alloc::string::String,
    /// An array containing the names of columns by which the data should be partitioned
    #[prost(string, repeated, tag="6")]
    pub partition_columns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The time when this metadata action is created, in milliseconds since the Unix epoch
    #[prost(int64, optional, tag="7")]
    pub created_time: ::core::option::Option<i64>,
    /// A map containing configuration options for the metadata action
    #[prost(map="string, string", tag="8")]
    pub options: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTableVersionRequest {
    /// The table name to query. It's case-insensitive.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The schema name to query. It's case-insensitive.
    #[prost(string, tag="2")]
    pub schema: ::prost::alloc::string::String,
    /// The share name to query. It's case-insensitive.
    #[prost(string, tag="3")]
    pub share: ::prost::alloc::string::String,
    /// The startingTimestamp of the query, a string in the  ISO8601 format, in the UTC timezone,
    /// such as 2022-01-01T00:00:00Z. the server needs to return the earliest table version at
    /// or after the provided timestamp, can be earlier than the timestamp of table version 0.
    #[prost(string, optional, tag="4")]
    pub starting_timestamp: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct GetTableVersionResponse {
    /// The table version that was requested.
    #[prost(int64, tag="1")]
    pub version: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTableMetadataRequest {
    /// The table name to query. It's case-insensitive.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The share name to query. It's case-insensitive.
    #[prost(string, tag="2")]
    pub share: ::prost::alloc::string::String,
    /// The schema name to query. It's case-insensitive.
    #[prost(string, tag="3")]
    pub schema: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryResponse {
    #[prost(oneof="query_response::Response", tags="1, 2")]
    pub response: ::core::option::Option<query_response::Response>,
}
/// Nested message and enum types in `QueryResponse`.
pub mod query_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        /// Parquet response format
        #[prost(message, tag="1")]
        Parquet(super::ParquetResponse),
        /// Delta response format
        #[prost(message, tag="2")]
        Delta(super::DeltaResponse),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParquetResponse {
    #[prost(message, repeated, tag="1")]
    pub entries: ::prost::alloc::vec::Vec<ParquetLogMessage>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParquetLogMessage {
    #[prost(oneof="parquet_log_message::Entry", tags="1, 2")]
    pub entry: ::core::option::Option<parquet_log_message::Entry>,
}
/// Nested message and enum types in `ParquetLogMessage`.
pub mod parquet_log_message {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Entry {
        #[prost(message, tag="1")]
        Protocol(super::ProtocolParquet),
        #[prost(message, tag="2")]
        Metadata(super::MetadataParquet),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct ProtocolParquet {
    /// The minimum version of the protocol that a client must implement
    /// in order to correctly read a Delta Lake table.
    #[prost(int32, tag="1")]
    pub min_reader_version: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetadataParquet {
    /// Unique identifier for this table
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// User-provided identifier for this table
    #[prost(string, optional, tag="2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// User-provided description for this table
    #[prost(string, optional, tag="3")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    /// Specification of the encoding for the files stored in the table
    #[prost(message, optional, tag="4")]
    pub format: ::core::option::Option<Format>,
    /// Schema of the table
    #[prost(string, tag="5")]
    pub schema_string: ::prost::alloc::string::String,
    /// An array containing the names of columns by which the data should be partitioned
    #[prost(string, repeated, tag="6")]
    pub partition_columns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeltaResponse {
    #[prost(message, repeated, tag="1")]
    pub entries: ::prost::alloc::vec::Vec<DeltaLogMessage>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeltaLogMessage {
    #[prost(oneof="delta_log_message::Entry", tags="1, 2")]
    pub entry: ::core::option::Option<delta_log_message::Entry>,
}
/// Nested message and enum types in `DeltaLogMessage`.
pub mod delta_log_message {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Entry {
        #[prost(message, tag="1")]
        Protocol(super::ProtocolDelta),
        #[prost(message, tag="2")]
        Metadata(super::MetadatDelta),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct ProtocolDelta {
    /// The minimum version of the protocol that a client must implement
    /// in order to correctly read a Delta Lake table.
    #[prost(int32, tag="1")]
    pub min_reader_version: i32,
    #[prost(int32, tag="2")]
    pub min_writer_version: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetadatDelta {
    /// Metadata from Delta protocol
    #[prost(message, optional, tag="1")]
    pub delta_metadata: ::core::option::Option<Metadata>,
    /// The table version the metadata corresponds to, returned when querying
    /// table data with a version or timestamp parameter, or cdf query
    /// with includeHistoricalMetadata set to true.
    #[prost(int64, optional, tag="2")]
    pub version: ::core::option::Option<i64>,
    /// The size of the table in bytes, will be returned if available in the delta log.
    #[prost(int64, optional, tag="3")]
    pub size: ::core::option::Option<i64>,
    /// The number of files in the table, will be returned if available in the delta log.
    #[prost(int64, optional, tag="4")]
    pub num_files: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSharesRequest {
    /// The maximum number of results per page that should be returned.
    #[prost(int32, optional, tag="1")]
    pub max_results: ::core::option::Option<i32>,
    /// Specifies a page token to use. Set pageToken to the nextPageToken returned
    /// by a previous list request to get the next page of results.
    #[prost(string, optional, tag="2")]
    pub page_token: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSharesResponse {
    /// The shares that were requested.
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<Share>,
    /// Token that can be used to retrieve the next page of shares.
    /// An empty or missing token means that no more shares are available for retrieval.
    #[prost(string, optional, tag="2")]
    pub next_page_token: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetShareRequest {
    /// The share name to query. It's case-insensitive.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSharingSchemasRequest {
    /// The share name to query. It's case-insensitive.
    #[prost(string, tag="1")]
    pub share: ::prost::alloc::string::String,
    /// The maximum number of results per page that should be returned.
    #[prost(int32, optional, tag="2")]
    pub max_results: ::core::option::Option<i32>,
    /// Specifies a page token to use. Set pageToken to the nextPageToken returned
    /// by a previous list request to get the next page of results.
    #[prost(string, optional, tag="3")]
    pub page_token: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSharingSchemasResponse {
    /// The schemas that were requested.
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<SharingSchema>,
    /// Token that can be used to retrieve the next page of schemas.
    /// An empty or missing token means that no more schemas are available for retrieval.
    #[prost(string, optional, tag="2")]
    pub next_page_token: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSchemaTablesRequest {
    /// The schema name to query. It's case-insensitive.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The share name to query. It's case-insensitive.
    #[prost(string, tag="2")]
    pub share: ::prost::alloc::string::String,
    /// The maximum number of results per page that should be returned.
    #[prost(int32, optional, tag="3")]
    pub max_results: ::core::option::Option<i32>,
    /// Specifies a page token to use. Set pageToken to the nextPageToken returned
    /// by a previous list request to get the next page of results.
    #[prost(string, optional, tag="4")]
    pub page_token: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSchemaTablesResponse {
    /// The tables that were requested.
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<SharingTable>,
    /// Token that can be used to retrieve the next page of tables.
    /// An empty or missing token means that no more tables are available for retrieval.
    #[prost(string, optional, tag="2")]
    pub next_page_token: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListShareTablesRequest {
    /// The share name to query. It's case-insensitive.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The maximum number of results per page that should be returned.
    #[prost(int32, optional, tag="2")]
    pub max_results: ::core::option::Option<i32>,
    /// Specifies a page token to use. Set pageToken to the nextPageToken returned
    /// by a previous list request to get the next page of results.
    #[prost(string, optional, tag="3")]
    pub page_token: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListShareTablesResponse {
    /// The tables that were requested.
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<SharingTable>,
    /// Token that can be used to retrieve the next page of tables.
    /// An empty or missing token means that no more tables are available for retrieval.
    #[prost(string, optional, tag="2")]
    pub next_page_token: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateShareRequest {
    #[prost(message, optional, tag="1")]
    pub share: ::core::option::Option<ShareInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteShareRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSharingSchemaRequest {
    #[prost(string, tag="1")]
    pub share: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub schema: ::core::option::Option<SharingSchemaInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSharingSchemaRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub share: ::prost::alloc::string::String,
}
include!("delta_sharing.sharing.v1.serde.rs");
// @@protoc_insertion_point(module)