//! Authorization policies.
//!
//! Policies are used to determine whether a recipient is allowed to perform a specific action on a
//! resource. The action is represented by a [`Permission`] and the resource is represented by a
//! [`Resource`]. The [`Decision`] represents whether the action is allowed or denied for the given
//! recipient.

use crate::error::Result;
use crate::{Decision, Permission, Policy, Resource};

/// Policy that always returns a constant decision.
///
/// This policy is mainly useful for testing and development, or servers that do not require
/// authorization checks - e.g. when deployed in a trusted environment.
pub struct ConstantPolicy<T> {
    decision: Decision,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Default for ConstantPolicy<T> {
    fn default() -> Self {
        Self {
            decision: Decision::Allow,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T> ConstantPolicy<T> {
    /// Create a new instance of [`ConstantPolicy`].
    ///
    /// The [`ConstantPolicy`] will always return the same decision for all authorization requests.
    ///
    /// # Example
    /// ```
    /// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
    /// use delta_sharing_core::{Policy, Resource, Permission, Decision};
    /// use delta_sharing_core::policies::ConstantPolicy;
    ///
    /// let policy = ConstantPolicy::new(Decision::Allow);
    /// let resource = Resource::share("test");
    /// let permission = Permission::Read;
    /// let recipient = &();
    ///
    /// let decision = policy.authorize(resource, permission, recipient).await.unwrap();
    /// assert_eq!(decision, Decision::Allow);
    /// # Ok(())
    /// # }
    /// ```
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
        Ok(self.decision)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn assert_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<ConstantPolicy<()>>();
    }

    #[tokio::test]
    async fn allow_by_default() {
        let policy = ConstantPolicy::default();

        let resource = Resource::Share("test_share".to_string());
        let permission = Permission::Read;
        let recipient = &();

        let decision = policy
            .authorize(resource, permission, recipient)
            .await
            .unwrap();
        assert_eq!(decision, Decision::Allow);
    }

    #[tokio::test]
    async fn allow() {
        let policy = ConstantPolicy::new(Decision::Allow);

        let resource = Resource::Share("test_share".to_string());
        let permission = Permission::Read;
        let recipient = &();

        let decision = policy
            .authorize(resource, permission, recipient)
            .await
            .unwrap();
        assert_eq!(decision, Decision::Allow);
    }

    #[tokio::test]
    async fn deny() {
        let policy = ConstantPolicy::new(Decision::Deny);

        let resource = Resource::Share("test_share".to_string());
        let permission = Permission::Read;
        let recipient = &();

        let decision = policy
            .authorize(resource, permission, recipient)
            .await
            .unwrap();
        assert_eq!(decision, Decision::Deny);
    }
}
