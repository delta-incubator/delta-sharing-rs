use crate::error::Result;

/// Permission that a policy can authorize.
#[derive(Debug, Clone)]
pub enum Permission {
    Read,
    Write,
    Manage,
}

/// Resource that a policy can authorize.
#[derive(Debug, Clone)]
pub enum Resource {
    Share(String),
    Schema(String),
    Table(String),
    File(String),
}

/// Decision made by a policy.
#[derive(Debug, Clone)]
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
        recipient: Self::Recipient,
    ) -> Result<Decision>;
}

/// Default recipient for delta sharing.
#[derive(Debug, Clone, PartialEq)]
pub enum RecipientId {
    Anonymous,
    User(String),
}
