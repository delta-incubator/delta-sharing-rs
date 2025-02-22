//! Authorization policies.
//!
//! Policies are used to determine whether a recipient is allowed to perform a specific action on a
//! resource. The action is represented by a [`Permission`] and the resource is represented by a
//! [`Resource`]. The [`Decision`] represents whether the action is allowed or denied for the given
//! recipient.

use bytes::Bytes;
use std::borrow::Borrow;
use std::ops::Deref;
use std::sync::Arc;

use crate::models::SecuredAction;
use crate::{Error, ObjectLabel, ResourceRef, Result};

pub use constant::*;

mod constant;

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

/// Permission that a policy can authorize.
#[derive(Debug, Clone)]
pub enum Permission {
    Read,
    Write,
    Manage,
    Create,
    Use,
}

impl AsRef<str> for Permission {
    fn as_ref(&self) -> &str {
        match self {
            Self::Read => "read",
            Self::Write => "write",
            Self::Manage => "manage",
            Self::Create => "create",
            Self::Use => "use",
        }
    }
}

impl From<Permission> for String {
    fn from(val: Permission) -> Self {
        val.as_ref().to_string()
    }
}

/// Resource that a policy can authorize.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResourceIdent {
    Share(ResourceRef),
    SharingSchema(ResourceRef),
    SharingTable(ResourceRef),
    Credential(ResourceRef),
    StorageLocation(ResourceRef),
    Catalog(ResourceRef),
    Schema(ResourceRef),
    Table(ResourceRef),
}

impl ResourceIdent {
    pub fn label(&self) -> &ObjectLabel {
        self.as_ref()
    }

    pub fn reference(&self) -> &ResourceRef {
        self.as_ref()
    }

    pub fn share(name: impl Into<ResourceRef>) -> Self {
        Self::Share(name.into())
    }

    pub fn sharing_schema(name: impl Into<ResourceRef>) -> Self {
        Self::SharingSchema(name.into())
    }

    pub fn sharing_table(name: impl Into<ResourceRef>) -> Self {
        Self::SharingTable(name.into())
    }

    pub fn credential(name: impl Into<ResourceRef>) -> Self {
        Self::Credential(name.into())
    }

    pub fn storage_location(name: impl Into<ResourceRef>) -> Self {
        Self::StorageLocation(name.into())
    }

    pub fn catalog(name: impl Into<ResourceRef>) -> Self {
        Self::Catalog(name.into())
    }

    pub fn schema(name: impl Into<ResourceRef>) -> Self {
        Self::Schema(name.into())
    }

    pub fn table(name: impl Into<ResourceRef>) -> Self {
        Self::Table(name.into())
    }
}

impl std::fmt::Display for ResourceIdent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceIdent::Share(r) => write!(f, "share:{}", r),
            ResourceIdent::SharingSchema(r) => write!(f, "schema:{}", r),
            ResourceIdent::SharingTable(r) => write!(f, "table:{}", r),
            ResourceIdent::Credential(r) => write!(f, "credential:{}", r),
            ResourceIdent::StorageLocation(r) => write!(f, "storage_location:{}", r),
            ResourceIdent::Catalog(r) => write!(f, "catalog:{}", r),
            ResourceIdent::Schema(r) => write!(f, "schema:{}", r),
            ResourceIdent::Table(r) => write!(f, "table:{}", r),
        }
    }
}

impl AsRef<ResourceRef> for ResourceIdent {
    fn as_ref(&self) -> &ResourceRef {
        match self {
            ResourceIdent::Share(r) => r,
            ResourceIdent::SharingSchema(r) => r,
            ResourceIdent::SharingTable(r) => r,
            ResourceIdent::Credential(r) => r,
            ResourceIdent::StorageLocation(r) => r,
            ResourceIdent::Catalog(r) => r,
            ResourceIdent::Schema(r) => r,
            ResourceIdent::Table(r) => r,
        }
    }
}

impl AsRef<ObjectLabel> for ResourceIdent {
    fn as_ref(&self) -> &ObjectLabel {
        match self {
            ResourceIdent::Share(_) => &ObjectLabel::ShareInfo,
            ResourceIdent::SharingSchema(_) => &ObjectLabel::SharingSchemaInfo,
            ResourceIdent::SharingTable(_) => &ObjectLabel::SharingTable,
            ResourceIdent::Credential(_) => &ObjectLabel::Credential,
            ResourceIdent::StorageLocation(_) => &ObjectLabel::StorageLocation,
            ResourceIdent::Catalog(_) => &ObjectLabel::CatalogInfo,
            ResourceIdent::Schema(_) => &ObjectLabel::SchemaInfo,
            ResourceIdent::Table(_) => &ObjectLabel::TableInfo,
        }
    }
}

impl Deref for ResourceIdent {
    type Target = ResourceRef;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl Borrow<ResourceRef> for ResourceIdent {
    fn borrow(&self) -> &ResourceRef {
        self.as_ref()
    }
}

impl Borrow<ResourceRef> for &ResourceIdent {
    fn borrow(&self) -> &ResourceRef {
        self.as_ref()
    }
}

impl From<ResourceIdent> for ResourceRef {
    fn from(ident: ResourceIdent) -> Self {
        match ident {
            ResourceIdent::Share(r) => r,
            ResourceIdent::SharingSchema(r) => r,
            ResourceIdent::SharingTable(r) => r,
            ResourceIdent::Credential(r) => r,
            ResourceIdent::StorageLocation(r) => r,
            ResourceIdent::Catalog(r) => r,
            ResourceIdent::Schema(r) => r,
            ResourceIdent::Table(r) => r,
        }
    }
}

impl From<&ResourceIdent> for ResourceRef {
    fn from(ident: &ResourceIdent) -> Self {
        (ident as &dyn AsRef<ResourceRef>).as_ref().clone()
    }
}

impl From<&ResourceIdent> for ObjectLabel {
    fn from(ident: &ResourceIdent) -> Self {
        (ident as &dyn AsRef<ObjectLabel>).as_ref().clone()
    }
}

impl From<ResourceIdent> for ObjectLabel {
    fn from(ident: ResourceIdent) -> Self {
        (&ident).into()
    }
}

pub trait AsResource {
    fn as_resource(&self) -> ResourceIdent;
}

impl<T: AsResource> AsResource for &T {
    fn as_resource(&self) -> ResourceIdent {
        (*self).as_resource()
    }
}

impl AsResource for ResourceIdent {
    fn as_resource(&self) -> ResourceIdent {
        self.clone()
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

/// Policy for access control.
#[async_trait::async_trait]
pub trait Policy: Send + Sync + 'static {
    async fn check(&self, obj: &dyn SecuredAction, recipient: &Recipient) -> Result<Decision> {
        self.authorize(&obj.resource(), obj.permission(), recipient)
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
        resource: &ResourceIdent,
        permission: &Permission,
        recipient: &Recipient,
    ) -> Result<Decision>;

    async fn authorize_many(
        &self,
        resources: &[ResourceIdent],
        permission: &Permission,
        recipient: &Recipient,
    ) -> Result<Vec<Decision>> {
        let mut decisions = Vec::with_capacity(resources.len());
        for resource in resources {
            decisions.push(self.authorize(resource, permission, recipient).await?);
        }
        Ok(decisions)
    }

    /// Check if the policy allows the action, and return an error if denied.
    async fn authorize_checked(
        &self,
        resource: &ResourceIdent,
        permission: &Permission,
        recipient: &Recipient,
    ) -> Result<()> {
        match self.authorize(resource, permission, recipient).await? {
            Decision::Allow => Ok(()),
            Decision::Deny => Err(Error::NotAllowed),
        }
    }
}

pub trait HasPolicy: Send + Sync + 'static {
    fn policy(&self) -> &Arc<dyn Policy>;
}

#[async_trait::async_trait]
impl<T: Policy> Policy for Arc<T> {
    async fn authorize(
        &self,
        resource: &ResourceIdent,
        permission: &Permission,
        recipient: &Recipient,
    ) -> Result<Decision> {
        T::authorize(self, resource, permission, recipient).await
    }

    async fn authorize_many(
        &self,
        resources: &[ResourceIdent],
        permission: &Permission,
        recipient: &Recipient,
    ) -> Result<Vec<Decision>> {
        T::authorize_many(self, resources, permission, recipient).await
    }
}

#[async_trait::async_trait]
impl<T: HasPolicy> Policy for T {
    async fn authorize(
        &self,
        resource: &ResourceIdent,
        permission: &Permission,
        recipient: &Recipient,
    ) -> Result<Decision> {
        self.policy()
            .authorize(resource, permission, recipient)
            .await
    }

    async fn authorize_many(
        &self,
        resources: &[ResourceIdent],
        permission: &Permission,
        recipient: &Recipient,
    ) -> Result<Vec<Decision>> {
        self.policy()
            .authorize_many(resources, permission, recipient)
            .await
    }
}

/// Checks if the recipient has the given permission for each resource,
/// and retains only those that receive an allow decision.
pub async fn process_resources<T: Policy + Sized, R: AsResource + Send>(
    handler: &T,
    recipient: &Recipient,
    permission: &Permission,
    resources: &mut Vec<R>,
) -> Result<()> {
    let res = resources
        .iter_mut()
        .map(|share| share.as_resource())
        .collect::<Vec<_>>();
    let mut decisions = handler.authorize_many(&res, permission, recipient).await?;
    resources.retain(|_| decisions.pop() == Some(Decision::Allow));
    Ok(())
}
