// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pagination {
    /// The maximum number of results per page that should be returned.
    /// If the number of available results is larger than maxResults, the response
    /// will provide a next_page_token that can be used to get the next page of results
    /// in subsequent list requests. The server may return fewer than maxResults
    /// items even if there are more available. The client should check nextPageToken
    /// in the response to determine if there are more available.
    /// Must be non-negative. 0 will return no results but nextPageToken may be populated.
    #[prost(int32, optional, tag="1")]
    pub max_results: ::core::option::Option<i32>,
    /// Specifies a page token to use. Set pageToken to the nextPageToken returned
    /// by a previous list request to get the next page of results.
    /// next_page_token will not be returned in a response if there are no more results available.
    #[prost(string, optional, tag="2")]
    pub page_token: ::core::option::Option<::prost::alloc::string::String>,
}
/// A share is a logical grouping to share with recipients. A share can be shared with one or multiple recipients.
/// A recipient can access all resources in a share. A share may contain multiple schemas.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Share {
    /// Unique identifier for the share.
    #[prost(string, optional, tag="1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    /// Name of the share.
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
}
/// A schema is a logical grouping of tables. A schema may contain multiple tables.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Schema {
    /// The name of the schama
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The share name that the schema belongs to.
    #[prost(string, tag="2")]
    pub share: ::prost::alloc::string::String,
}
/// A table is a Delta Lake table or a view on top of a Delta Lake table.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Table {
    /// Unique identifier for the table.
    #[prost(string, optional, tag="1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    /// The name of the table.
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// The schema name that the table belongs to.
    #[prost(string, tag="3")]
    pub schema: ::prost::alloc::string::String,
    /// The share name that the table belongs to.
    #[prost(string, tag="4")]
    pub share: ::prost::alloc::string::String,
    /// A unique identifier for the share this table belongs to.
    #[prost(string, optional, tag="5")]
    pub share_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSharesRequest {
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<Pagination>,
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
    pub share: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetShareResponse {
    /// The share that was requested.
    #[prost(message, optional, tag="1")]
    pub share: ::core::option::Option<Share>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSchemasRequest {
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<Pagination>,
    /// The share name to query. It's case-insensitive.
    #[prost(string, tag="2")]
    pub share: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSchemasResponse {
    /// The schemas that were requested.
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<Schema>,
    /// Token that can be used to retrieve the next page of schemas.
    /// An empty or missing token means that no more schemas are available for retrieval.
    #[prost(string, optional, tag="2")]
    pub next_page_token: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSchemaTablesRequest {
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<Pagination>,
    /// The share name to query. It's case-insensitive.
    #[prost(string, tag="2")]
    pub share: ::prost::alloc::string::String,
    /// The schema name to query. It's case-insensitive.
    #[prost(string, tag="3")]
    pub schema: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSchemaTablesResponse {
    /// The tables that were requested.
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<Table>,
    /// Token that can be used to retrieve the next page of tables.
    /// An empty or missing token means that no more tables are available for retrieval.
    #[prost(string, optional, tag="2")]
    pub next_page_token: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListShareTablesRequest {
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<Pagination>,
    /// The share name to query. It's case-insensitive.
    #[prost(string, tag="2")]
    pub share: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListShareTablesResponse {
    /// The tables that were requested.
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<Table>,
    /// Token that can be used to retrieve the next page of tables.
    /// An empty or missing token means that no more tables are available for retrieval.
    #[prost(string, optional, tag="2")]
    pub next_page_token: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTableVersionRequest {
    /// The share name to query. It's case-insensitive.
    #[prost(string, tag="1")]
    pub share: ::prost::alloc::string::String,
    /// The schema name to query. It's case-insensitive.
    #[prost(string, tag="2")]
    pub schema: ::prost::alloc::string::String,
    /// The table name to query. It's case-insensitive.
    #[prost(string, tag="3")]
    pub table: ::prost::alloc::string::String,
    /// The startingTimestamp of the query, a string in the  ISO8601 format, in the UTC timezone,
    /// such as 2022-01-01T00:00:00Z. the server needs to return the earliest table version at
    /// or after the provided timestamp, can be earlier than the timestamp of table version 0.
    #[prost(string, optional, tag="4")]
    pub starting_timestamp: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTableVersionResponse {
    /// The table version that was requested.
    #[prost(int64, tag="1")]
    pub version: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Profile {
    /// The file format version of the profile file. This version will be increased whenever
    /// non-forward-compatible changes are made to the profile format. When a client is running
    /// an unsupported profile file format version, it should show an error message instructing
    /// the user to upgrade to a newer version of their client.
    #[prost(int32, tag="1")]
    pub share_credentials_version: i32,
    /// The url of the sharing server.
    #[prost(string, tag="2")]
    pub endpoint: ::prost::alloc::string::String,
    /// The bearer token to access the server.
    #[prost(string, tag="3")]
    pub bearer_token: ::prost::alloc::string::String,
    /// The expiration time of the bearer token in ISO 8601 format. This field is optional
    /// and if it is not provided, the bearer token can be seen as never expire.
    #[prost(string, optional, tag="4")]
    pub expiration_time: ::core::option::Option<::prost::alloc::string::String>,
}
include!("delta_sharing.v1.serde.rs");
include!("delta_sharing.v1.tonic.rs");
// @@protoc_insertion_point(module)