use std::sync::Arc;

use bytes::Bytes;
use chrono::{DateTime, Utc};
use jsonwebtoken::Validation;
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
pub mod capabilities;
pub mod error;
mod grpc;
#[cfg(feature = "memory")]
mod in_memory;
mod kernel;
pub mod policies;
#[cfg(feature = "profiles")]
mod profiles;
#[cfg(feature = "axum")]
mod rest;

pub use error::*;
#[cfg(feature = "memory")]
pub use in_memory::*;
pub use kernel::*;
pub use policies::*;
#[cfg(feature = "profiles")]
pub use profiles::*;
pub use types::*;

#[derive(Clone, Debug)]
pub struct Recipient(pub Bytes);

#[derive(Clone)]
pub struct DeltaSharingHandler {
    pub discovery: Arc<dyn DiscoveryHandler>,
    pub query: Arc<dyn TableQueryHandler>,
    pub policy: Arc<dyn Policy>,
}

/// Handler for discovering shares, schemas, and tables exposed by a Delta Sharing server.
#[async_trait::async_trait]
pub trait DiscoveryHandler: Send + Sync + 'static {
    /// List all shares that the recipient is allowed to read.
    async fn list_shares(
        &self,
        request: ListSharesRequest,
        recipient: &Recipient,
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

impl AsRef<str> for Permission {
    fn as_ref(&self) -> &str {
        match self {
            Self::Read => "read",
            Self::Write => "write",
            Self::Manage => "manage",
        }
    }
}

impl Into<String> for Permission {
    fn into(self) -> String {
        self.as_ref().to_string()
    }
}

/// Resource that a policy can authorize.
#[derive(Debug, Clone, PartialEq)]
pub enum Resource {
    Share(String),
    Schema(String),
    Table(String),
    File(String),
    Profiles,
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

impl Into<String> for &Resource {
    fn into(self) -> String {
        match self {
            Resource::Share(s) => format!("share::{s}"),
            Resource::Schema(s) => format!("schema::{s}"),
            Resource::Table(t) => format!("table::{t}"),
            Resource::File(f) => format!("file::{f}"),
            Resource::Profiles => "profiles".to_string(),
        }
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
    /// Check if the policy allows the action.
    ///
    /// Specifically, this method should return [`Decision::Allow`] if the recipient
    /// is granted the requested permission on the resource, and [`Decision::Deny`] otherwise.
    async fn authorize(
        &self,
        resource: Resource,
        permission: Permission,
        recipient: &Recipient,
    ) -> Result<Decision>;

    async fn authorize_checked(
        &self,
        resource: Resource,
        permission: Permission,
        recipient: &Recipient,
    ) -> Result<()> {
        match self.authorize(resource, permission, recipient).await? {
            Decision::Allow => Ok(()),
            Decision::Deny => Err(Error::NotAllowed),
        }
    }

    async fn authorize_share(
        &self,
        share: String,
        permission: Permission,
        recipient: &Recipient,
    ) -> Result<()> {
        self.authorize_checked(Resource::Share(share), permission, recipient)
            .await
    }
}

/// Claims that are encoded in a profile.
pub trait ProfileClaims: Serialize + DeserializeOwned + Send + Sync {
    /// Get the profile fingerprint from the claims.
    fn fingerprint(&self) -> String;

    fn validation() -> Validation {
        Validation::default()
    }
}

#[async_trait::async_trait]
pub trait ProfileManager: Send + Sync {
    /// Claims that are encoded in the profile.
    type Claims: ProfileClaims;

    /// Issue a profile for a set of claims that can be shared with a recipient.
    async fn issue_profile(
        &self,
        claims: &Self::Claims,
        expiration_time: Option<DateTime<Utc>>,
    ) -> Result<Profile>;

    /// Revoke a profile by its fingerprint.
    ///
    /// This should invalidate the profile and prevent it from being used.
    async fn revoke_profile(&self, fingerprint: &str) -> Result<()>;

    /// Validate a profile token and return the claims.
    /// This should return an error if the profile is invalid or has been revoked.
    async fn validate_profile(&self, token: &str) -> Result<Self::Claims>;
}
