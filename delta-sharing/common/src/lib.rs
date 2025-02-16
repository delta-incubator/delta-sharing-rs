use std::sync::Arc;

use bytes::Bytes;

pub mod error;
#[cfg(feature = "grpc")]
mod grpc;
#[cfg(feature = "memory")]
mod in_memory;
mod kernel;
pub mod models;
pub mod policies;
#[cfg(feature = "axum")]
pub mod rest;

pub use error::*;
#[cfg(feature = "memory")]
pub use in_memory::*;
pub use kernel::*;
pub use models::v1::*;
use models::AsResource;
pub use policies::*;

#[derive(Clone, Debug)]
pub enum Recipient {
    Anonymous,
    User(String),
    Custom(Bytes),
}

impl Recipient {
    pub fn anonymous() -> Self {
        Self::Anonymous
    }

    pub fn user(name: impl Into<String>) -> Self {
        Self::User(name.into())
    }

    pub fn custom(data: Bytes) -> Self {
        Self::Custom(data)
    }
}

/// Unique identifier for a resource.
#[derive(Debug, Clone, PartialEq)]
pub enum ResourceIdent {
    Uunid(uuid::Uuid),
    Name(Vec<String>, String),
}

impl std::fmt::Display for ResourceIdent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Uunid(u) => write!(f, "{}", u),
            Self::Name(path, name) => {
                if path.is_empty() {
                    write!(f, "{}", name)
                } else {
                    write!(f, "{}.{}", path.join("."), name)
                }
            }
        }
    }
}

impl From<uuid::Uuid> for ResourceIdent {
    fn from(val: uuid::Uuid) -> Self {
        Self::Uunid(val)
    }
}

impl From<String> for ResourceIdent {
    fn from(val: String) -> Self {
        Self::Name(vec![], val)
    }
}

impl From<&String> for ResourceIdent {
    fn from(val: &String) -> Self {
        Self::Name(vec![], val.clone())
    }
}

impl From<&str> for ResourceIdent {
    fn from(val: &str) -> Self {
        Self::Name(vec![], val.to_string())
    }
}

impl<T: ToString + Sized, U: ToString, const N: usize> From<([T; N], U)> for ResourceIdent {
    fn from(val: ([T; N], U)) -> Self {
        Self::Name(
            val.0.iter().map(|s| s.to_string()).collect(),
            val.1.to_string(),
        )
    }
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
    async fn get_share(&self, request: GetShareRequest) -> Result<Share>;

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

#[async_trait::async_trait]
impl<T: DiscoveryHandler> DiscoveryHandler for Arc<T> {
    async fn list_shares(
        &self,
        request: ListSharesRequest,
        recipient: &Recipient,
    ) -> Result<ListSharesResponse> {
        T::list_shares(self, request, recipient).await
    }

    async fn get_share(&self, request: GetShareRequest) -> Result<Share> {
        T::get_share(self, request).await
    }

    async fn list_schemas(&self, request: ListSchemasRequest) -> Result<ListSchemasResponse> {
        T::list_schemas(self, request).await
    }

    async fn list_schema_tables(
        &self,
        request: ListSchemaTablesRequest,
    ) -> Result<ListSchemaTablesResponse> {
        T::list_schema_tables(self, request).await
    }

    async fn list_share_tables(
        &self,
        request: ListShareTablesRequest,
    ) -> Result<ListShareTablesResponse> {
        T::list_share_tables(self, request).await
    }
}

/// Resolver for the storage location of a table.
#[async_trait::async_trait]
pub trait TableLocationResover: Send + Sync {
    async fn resolve(&self, table: &models::TableRef) -> Result<url::Url>;
}

#[async_trait::async_trait]
impl<T: TableLocationResover> TableLocationResover for Arc<T> {
    async fn resolve(&self, table: &models::TableRef) -> Result<url::Url> {
        T::resolve(self, table).await
    }
}

/// Handler for querying tables exposed by a Delta Sharing server.
#[async_trait::async_trait]
pub trait TableQueryHandler: Send + Sync {
    async fn get_table_version(
        &self,
        request: GetTableVersionRequest,
    ) -> Result<GetTableVersionResponse>;

    async fn get_table_metadata(&self, request: GetTableMetadataRequest) -> Result<QueryResponse>;
}

#[async_trait::async_trait]
impl<T: TableQueryHandler> TableQueryHandler for Arc<T> {
    async fn get_table_version(
        &self,
        request: GetTableVersionRequest,
    ) -> Result<GetTableVersionResponse> {
        T::get_table_version(self, request).await
    }

    async fn get_table_metadata(&self, request: GetTableMetadataRequest) -> Result<QueryResponse> {
        T::get_table_metadata(self, request).await
    }
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

impl From<Permission> for String {
    fn from(val: Permission) -> Self {
        val.as_ref().to_string()
    }
}

/// Resource that a policy can authorize.
#[derive(Debug, Clone, PartialEq)]
pub enum Resource {
    Share(ResourceIdent),
    Schema(ResourceIdent),
    Table(ResourceIdent),
    File(ResourceIdent),
    Profiles,
}

impl Resource {
    pub fn share(name: impl Into<ResourceIdent>) -> Self {
        Self::Share(name.into())
    }

    pub fn schema(name: impl Into<ResourceIdent>) -> Self {
        Self::Schema(name.into())
    }

    pub fn table(name: impl Into<ResourceIdent>) -> Self {
        Self::Table(name.into())
    }

    pub fn file(name: impl Into<ResourceIdent>) -> Self {
        Self::File(name.into())
    }
}

impl AsResource for Resource {
    fn as_resource(&self) -> Resource {
        self.clone()
    }
}

impl From<&Resource> for String {
    fn from(val: &Resource) -> Self {
        match val {
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

pub trait SecuredAction: Send + Sync {
    fn resource(&self) -> Resource;
    fn permission(&self) -> Permission;
}

/// Policy for access control.
#[async_trait::async_trait]
pub trait Policy: Send + Sync {
    async fn check(&self, obj: &dyn SecuredAction, recipient: &Recipient) -> Result<Decision> {
        self.authorize(&obj.resource(), &obj.permission(), recipient)
            .await
    }

    async fn check_required(&self, obj: &dyn SecuredAction, recipient: &Recipient) -> Result<()> {
        match self.check(obj, recipient).await? {
            Decision::Allow => Ok(()),
            Decision::Deny => Err(Error::NotAllowed),
        }
    }

    /// Check if the policy allows the action.
    ///
    /// Specifically, this method should return [`Decision::Allow`] if the recipient
    /// is granted the requested permission on the resource, and [`Decision::Deny`] otherwise.
    async fn authorize(
        &self,
        resource: &Resource,
        permission: &Permission,
        recipient: &Recipient,
    ) -> Result<Decision>;

    async fn authorize_many(
        &self,
        resources: &[Resource],
        permission: &Permission,
        recipient: &Recipient,
    ) -> Result<Vec<Decision>> {
        let mut decisions = Vec::with_capacity(resources.len());
        for resource in resources {
            decisions.push(self.authorize(resource, permission, recipient).await?);
        }
        Ok(decisions)
    }

    async fn authorize_checked(
        &self,
        resource: &Resource,
        permission: &Permission,
        recipient: &Recipient,
    ) -> Result<()> {
        match self.authorize(resource, &permission, recipient).await? {
            Decision::Allow => Ok(()),
            Decision::Deny => Err(Error::NotAllowed),
        }
    }

    async fn authorize_share(
        &self,
        share: String,
        permission: &Permission,
        recipient: &Recipient,
    ) -> Result<()> {
        self.authorize_checked(&Resource::share(share), permission, recipient)
            .await
    }
}

#[async_trait::async_trait]
impl<T: Policy> Policy for Arc<T> {
    async fn authorize(
        &self,
        resource: &Resource,
        permission: &Permission,
        recipient: &Recipient,
    ) -> Result<Decision> {
        T::authorize(self, resource, permission, recipient).await
    }
}

/// Checks if the recipient has the given permission for each resource,
/// and retains only those that receive an allow decision.
pub async fn process_resources<T: Policy, R: AsResource>(
    handler: &T,
    recipient: &Recipient,
    permission: &Permission,
    resources: &mut Vec<R>,
) -> Result<()> {
    let res = resources
        .into_iter()
        .map(|share| share.as_resource())
        .collect::<Vec<_>>();
    let mut decisions = handler.authorize_many(&res, permission, recipient).await?;
    resources.retain(|_| decisions.pop() == Some(Decision::Allow));
    Ok(())
}
