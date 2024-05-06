use serde::{de::DeserializeOwned, Serialize};

#[allow(dead_code)]
pub mod types {
    use serde::Serialize;

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

    include!("gen/delta_sharing.v1.rs");
}
pub mod error;
#[cfg(feature = "memory")]
mod in_memory;
mod kernel;
pub mod policies;
#[cfg(feature = "profiles")]
mod profiles;

pub use error::*;
#[cfg(feature = "memory")]
pub use in_memory::*;
pub use kernel::*;
pub use policies::*;
#[cfg(feature = "profiles")]
pub use profiles::*;
pub use types::*;

/// Handler for discovering shares, schemas, and tables exposed by a Delta Sharing server.
#[async_trait::async_trait]
pub trait DiscoveryHandler: Send + Sync {
    type Recipient: Send;

    /// List all shares that the recipient is allowed to read.
    async fn list_shares(
        &self,
        request: ListSharesRequest,
        recipient: Self::Recipient,
    ) -> Result<ListSharesResponse>;

    /// Get a share by name.
    async fn get_share(&self, request: GetShareRequest) -> Result<GetShareResponse>;

    /// List all schemas in a share.
    async fn list_schemas(&self, request: ListSchemasRequest) -> Result<ListSchemasResponse>;

    /// List all tables in a schema.
    async fn list_schema_tables(
        &self,
        request: ListSchemaTablesRequest,
    ) -> Result<ListSchemaTablesResponse>;

    /// List all tables in a share.
    async fn list_share_tables(
        &self,
        request: ListShareTablesRequest,
    ) -> Result<ListShareTablesResponse>;
}

/// Resolver for the storage location of a table.
#[async_trait::async_trait]
pub trait TableLocationResover: Send + Sync {
    async fn resolve(&self, table: &types::TableRef) -> Result<url::Url>;
}

/// Handler for querying tables exposed by a Delta Sharing server.
#[async_trait::async_trait]
pub trait TableQueryHandler: Send + Sync {
    async fn get_table_version(
        &self,
        request: GetTableVersionRequest,
    ) -> Result<GetTableVersionResponse>;
}

/// Permission that a policy can authorize.
#[derive(Debug, Clone)]
pub enum Permission {
    Read,
    Write,
    Manage,
}

/// Resource that a policy can authorize.
#[derive(Debug, Clone, PartialEq)]
pub enum Resource {
    Share(String),
    Schema(String),
    Table(String),
    File(String),
}

impl Resource {
    pub fn share(name: impl Into<String>) -> Self {
        Self::Share(name.into())
    }

    pub fn schema(name: impl Into<String>) -> Self {
        Self::Schema(name.into())
    }

    pub fn table(name: impl Into<String>) -> Self {
        Self::Table(name.into())
    }

    pub fn file(name: impl Into<String>) -> Self {
        Self::File(name.into())
    }
}

/// Decision made by a policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decision {
    /// Allow the action.
    Allow,
    /// Deny the action.
    Deny,
}

/// Authenticator for authenticating requests to a sharing server.
pub trait Authenticator: Send + Sync {
    type Request;
    type Recipient: Send;

    /// Authenticate a request.
    ///
    /// This method should return the recipient of the request, or an error if the request
    /// is not authenticated or the recipient cannot be determined from the request.
    fn authenticate(&self, request: &Self::Request) -> Result<Self::Recipient>;
}

/// Policy for access control.
#[async_trait::async_trait]
pub trait Policy: Send + Sync {
    type Recipient: Send;

    /// Check if the policy allows the action.
    ///
    /// Specifically, this method should return [`Decision::Allow`] if the recipient
    /// is granted the requested permission on the resource, and [`Decision::Deny`] otherwise.
    async fn authorize(
        &self,
        resource: Resource,
        permission: Permission,
        recipient: &Self::Recipient,
    ) -> Result<Decision>;
}

/// Claims that are encoded in a profile.
pub trait ProfileClaims: Serialize + DeserializeOwned + Send + Sync {
    /// Get the profile fingerprint from the claims.
    fn fingerprint(&self) -> String;
}

#[async_trait::async_trait]
pub trait ProfileManager: Send + Sync {
    /// Claims that are encoded in the profile.
    type Claims: ProfileClaims;

    /// Issue a profile for a set of claims that can be shared with a recipient.
    async fn issue_profile(&self, claims: &Self::Claims) -> Result<Profile>;

    /// Revoke a profile by its fingerprint.
    ///
    /// This should invalidate the profile and prevent it from being used.
    async fn revoke_profile(&self, fingerprint: String) -> Result<()>;
}
