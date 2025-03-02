//! Authorization policies.
//!
//! Policies are used to determine whether a recipient is allowed to perform a specific action on a
//! resource. The action is represented by a [`Permission`] and the resource is represented by a
//! [`Resource`]. The [`Decision`] represents whether the action is allowed or denied for the given
//! recipient.

use bytes::Bytes;
use std::sync::Arc;
use strum::AsRefStr;

use crate::api::SecuredAction;
use crate::{Error, ResourceExt, ResourceIdent, Result};

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
#[derive(Debug, Clone, AsRefStr, PartialEq, Eq, strum::EnumString)]
#[strum(serialize_all = "snake_case", ascii_case_insensitive)]
pub enum Permission {
    Read,
    Write,
    Manage,
    Create,
    Use,
    Browse,
    Select,
}

impl From<Permission> for String {
    fn from(val: Permission) -> Self {
        val.as_ref().to_string()
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

pub trait ProvidesPolicy: Send + Sync + 'static {
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
impl<T: ProvidesPolicy> Policy for T {
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
pub async fn process_resources<T: Policy + Sized, R: ResourceExt + Send>(
    handler: &T,
    recipient: &Recipient,
    permission: &Permission,
    resources: &mut Vec<R>,
) -> Result<()> {
    let res = resources.iter().map(|r| r.into()).collect::<Vec<_>>();
    let mut decisions = handler.authorize_many(&res, permission, recipient).await?;
    resources.retain(|_| decisions.pop() == Some(Decision::Allow));
    Ok(())
}
