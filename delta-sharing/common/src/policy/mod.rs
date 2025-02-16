//! Authorization policies.
//!
//! Policies are used to determine whether a recipient is allowed to perform a specific action on a
//! resource. The action is represented by a [`Permission`] and the resource is represented by a
//! [`Resource`]. The [`Decision`] represents whether the action is allowed or denied for the given
//! recipient.

use std::sync::Arc;

use crate::{Error, Recipient, ResourceIdent, Result};

pub use constant::*;

mod constant;

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

pub trait AsResource {
    fn as_resource(&self) -> Resource;
}

impl<T: AsResource> AsResource for &T {
    fn as_resource(&self) -> Resource {
        (*self).as_resource()
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
