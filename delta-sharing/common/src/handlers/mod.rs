use std::sync::Arc;

use crate::policy::Policy;
use crate::resources::ResourceStore;
use crate::{
    ProvidesPolicy, ProvidesResourceStore, ProvidesSecretManager, ResourceRef, Result,
    SecretManager, SharingQueryHandler, TableLocationResolver,
};

mod catalog;
mod credentials;
mod external_locations;
mod schemas;
mod sharing;

#[derive(Clone)]
pub struct ServerHandler {
    pub policy: Arc<dyn Policy>,
    pub store: Arc<dyn ResourceStore>,
    pub query: Arc<dyn SharingQueryHandler>,
    pub secrets: Arc<dyn SecretManager>,
}

impl ServerHandler {
    pub fn new(
        policy: Arc<dyn Policy>,
        store: Arc<dyn ResourceStore>,
        query: Arc<dyn SharingQueryHandler>,
        secrets: Arc<dyn SecretManager>,
    ) -> Self {
        Self {
            policy,
            store,
            query,
            secrets,
        }
    }
}

impl ProvidesPolicy for ServerHandler {
    fn policy(&self) -> &Arc<dyn Policy> {
        &self.policy
    }
}

impl ProvidesResourceStore for ServerHandler {
    fn store(&self) -> &dyn ResourceStore {
        self.store.as_ref()
    }
}

impl ProvidesSecretManager for ServerHandler {
    fn secret_manager(&self) -> &dyn SecretManager {
        self.secrets.as_ref()
    }
}

#[async_trait::async_trait]
impl TableLocationResolver for ServerHandler {
    async fn resolve(&self, _table: &ResourceRef) -> Result<url::Url> {
        todo!("resolve table location")
    }
}
