use std::sync::Arc;

use crate::policy::Policy;
use crate::resources::ResourceStore;
use crate::{
    ProvidesPolicy, ProvidesResourceStore, ProvidesSecretManager, ResourceRef, Result,
    SecretManager, TableLocationResolver,
};

use crate::api::{RequestContext, SharingQueryHandler};
use crate::models::sharing::v1::*;

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

#[async_trait::async_trait]
impl SharingQueryHandler for ServerHandler {
    async fn get_table_version(
        &self,
        request: GetTableVersionRequest,
        context: RequestContext,
    ) -> Result<GetTableVersionResponse> {
        self.check_required(&request, context.recipient()).await?;
        self.query.get_table_version(request, context).await
    }

    async fn get_table_metadata(
        &self,
        request: GetTableMetadataRequest,
        context: RequestContext,
    ) -> Result<QueryResponse> {
        self.check_required(&request, context.recipient()).await?;
        self.query.get_table_metadata(request, context).await
    }
}
