use crate::error::Result;

mod tokens;

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
    Allow,
    Deny,
}

/// Authenticator for authenticating requests to a sharing server.
pub trait Authenticator: Send + Sync {
    type Request;
    type Recipient: Send;

    /// Authenticate a request.
    fn authenticate(&self, request: &Self::Request) -> Result<Self::Recipient>;
}

/// Policy for access control.
#[async_trait::async_trait]
pub trait Policy: Send + Sync {
    type Recipient: Send;

    /// Check if the policy allows the action.
    async fn authorize(
        &self,
        resource: Resource,
        permission: Permission,
        recipient: &Self::Recipient,
    ) -> Result<Decision>;
}

/// Default recipient for delta sharing.
#[derive(Debug, Clone, PartialEq)]
pub enum RecipientId {
    Anonymous,
    User(String),
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
