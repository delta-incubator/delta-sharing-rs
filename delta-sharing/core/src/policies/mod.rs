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

pub struct AlwaysAllowPolicy<T: Send + Sync> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Send + Sync> AlwaysAllowPolicy<T> {
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

#[async_trait::async_trait]
impl<T: Send + Sync> Policy for AlwaysAllowPolicy<T> {
    type Recipient = T;

    async fn authorize(
        &self,
        _resource: Resource,
        _permission: Permission,
        _recipient: &Self::Recipient,
    ) -> Result<Decision> {
        Ok(Decision::Allow)
    }
}
