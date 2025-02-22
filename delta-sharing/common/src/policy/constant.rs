use crate::error::Result;
use crate::{Decision, Permission, Policy, Recipient, ResourceIdent};

/// Policy that always returns a constant decision.
///
/// This policy is mainly useful for testing and development, or servers that do not require
/// authorization checks - e.g. when deployed in a trusted environment.
pub struct ConstantPolicy {
    decision: Decision,
}

impl Default for ConstantPolicy {
    fn default() -> Self {
        Self {
            decision: Decision::Allow,
        }
    }
}

impl ConstantPolicy {
    /// Create a new instance of [`ConstantPolicy`].
    ///
    /// The [`ConstantPolicy`] will always return the same decision for all authorization requests.
    ///
    /// # Example
    /// ```
    /// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
    /// use delta_sharing_common::{Policy, Resource, Permission, Decision, Recipient};
    /// use delta_sharing_common::policies::ConstantPolicy;
    ///
    /// let policy = ConstantPolicy::new(Decision::Allow);
    /// let resource = Resource::share("test");
    /// let permission = Permission::Read;
    /// let recipient = Recipient::anonymous();
    ///
    /// let decision = policy.authorize(resource, permission, &recipient).await.unwrap();
    /// assert_eq!(decision, Decision::Allow);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(decision: Decision) -> Self {
        Self { decision }
    }
}

#[async_trait::async_trait]
impl Policy for ConstantPolicy {
    async fn authorize(
        &self,
        _: &ResourceIdent,
        _: &Permission,
        _: &Recipient,
    ) -> Result<Decision> {
        Ok(self.decision)
    }
}

#[cfg(test)]
mod test {
    use crate::resource_name;

    use super::*;

    #[test]
    fn assert_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<ConstantPolicy>();
    }

    #[tokio::test]
    async fn allow_by_default() {
        let policy = ConstantPolicy::default();

        let resource = ResourceIdent::share(resource_name!("test_share"));
        let permission = Permission::Read;
        let recipient = &Recipient::anonymous();

        let decision = policy
            .authorize(&resource, &permission, recipient)
            .await
            .unwrap();
        assert_eq!(decision, Decision::Allow);
    }

    #[tokio::test]
    async fn allow() {
        let policy = ConstantPolicy::new(Decision::Allow);

        let resource = ResourceIdent::share(resource_name!("test_share"));
        let permission = Permission::Read;
        let recipient = &Recipient::anonymous();

        let decision = policy
            .authorize(&resource, &permission, recipient)
            .await
            .unwrap();
        assert_eq!(decision, Decision::Allow);
    }

    #[tokio::test]
    async fn deny() {
        let policy = ConstantPolicy::new(Decision::Deny);

        let resource = ResourceIdent::share(resource_name!("test_share"));
        let permission = Permission::Read;
        let recipient = &Recipient::anonymous();

        let decision = policy
            .authorize(&resource, &permission, recipient)
            .await
            .unwrap();
        assert_eq!(decision, Decision::Deny);
    }
}
