use serde::{de::DeserializeOwned, Serialize};

use crate::error::Result;
use crate::types as t;
pub use profile::*;
pub use tokens::*;

mod profile;
mod tokens;

pub type DeltaRecipient<C = DefaultClaims> = DefaultRecipient<C>;

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

/// Decision made by a policy.
#[derive(Debug, Clone, PartialEq)]
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
    async fn issue_profile(&self, claims: &Self::Claims) -> Result<t::Profile>;

    /// Revoke a profile by its fingerprint.
    ///
    /// This should invalidate the profile and prevent it from being used.
    async fn revoke_profile(&self, fingerprint: String) -> Result<()>;
}

/// Policy that always returns a constant decision.
///
/// This policy is mainly useful for testing and development, or servers that do not require
/// authorization checks - e.g. when deployed in a trusted environment.
pub struct ConstantPolicy<T: Send + Sync> {
    decision: Decision,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Send + Sync> Default for ConstantPolicy<T> {
    fn default() -> Self {
        Self {
            decision: Decision::Allow,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: Send + Sync> ConstantPolicy<T> {
    /// Create a new instance of [`ConstantPolicy`].
    pub fn new(decision: Decision) -> Self {
        Self {
            decision,
            _phantom: std::marker::PhantomData,
        }
    }
}

#[async_trait::async_trait]
impl<T: Send + Sync> Policy for ConstantPolicy<T> {
    type Recipient = T;

    async fn authorize(&self, _: Resource, _: Permission, _: &Self::Recipient) -> Result<Decision> {
        Ok(self.decision.clone())
    }
}
