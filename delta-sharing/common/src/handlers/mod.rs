use std::sync::Arc;

use crate::policy::Policy;
use crate::resources::ResourceStore;
use crate::{
    Decision, Permission, Recipient, ResourceIdent, ResourceRef, Result, TableLocationResover,
    TableQueryHandler,
};

mod catalog;
mod sharing;

pub use sharing::*;

#[derive(Clone)]
pub struct ServerHandler {
    pub policy: Arc<dyn Policy>,
    pub store: Arc<dyn ResourceStore>,
    pub query: Arc<dyn TableQueryHandler>,
}

impl ServerHandler {
    pub fn new(
        policy: Arc<dyn Policy>,
        store: Arc<dyn ResourceStore>,
        query: Arc<dyn TableQueryHandler>,
    ) -> Self {
        Self {
            policy,
            store,
            query,
        }
    }
}

#[async_trait::async_trait]
impl Policy for ServerHandler {
    async fn authorize(
        &self,
        resource: &ResourceIdent,
        permission: &Permission,
        recipient: &Recipient,
    ) -> Result<Decision> {
        self.policy.authorize(resource, permission, recipient).await
    }
}

#[async_trait::async_trait]
impl TableLocationResover for ServerHandler {
    async fn resolve(&self, _table: &ResourceRef) -> Result<url::Url> {
        todo!("resolve table location")
    }
}
