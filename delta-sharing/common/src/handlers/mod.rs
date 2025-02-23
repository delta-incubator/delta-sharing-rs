use std::sync::Arc;

use crate::policy::Policy;
use crate::resources::ResourceStore;
use crate::{
    HasPolicy, ProvidesResourceStore, ResourceRef, Result, SharingQueryHandler,
    TableLocationResover,
};

mod catalog;
mod credentials;
mod sharing;

#[derive(Clone)]
pub struct ServerHandler {
    pub policy: Arc<dyn Policy>,
    pub store: Arc<dyn ResourceStore>,
    pub query: Arc<dyn SharingQueryHandler>,
}

impl ServerHandler {
    pub fn new(
        policy: Arc<dyn Policy>,
        store: Arc<dyn ResourceStore>,
        query: Arc<dyn SharingQueryHandler>,
    ) -> Self {
        Self {
            policy,
            store,
            query,
        }
    }
}

impl HasPolicy for ServerHandler {
    fn policy(&self) -> &Arc<dyn Policy> {
        &self.policy
    }
}

impl ProvidesResourceStore for ServerHandler {
    fn store(&self) -> &dyn ResourceStore {
        self.store.as_ref()
    }
}

#[async_trait::async_trait]
impl TableLocationResover for ServerHandler {
    async fn resolve(&self, _table: &ResourceRef) -> Result<url::Url> {
        todo!("resolve table location")
    }
}
