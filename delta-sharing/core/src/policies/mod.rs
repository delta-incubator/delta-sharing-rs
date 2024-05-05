use crate::error::Result;
use crate::{Decision, Permission, Policy, Resource};

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
