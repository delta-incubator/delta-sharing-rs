use std::collections::HashMap;
use std::sync::Arc;

use buoyant_kernel_engine::executor::tokio::{TokioBackgroundExecutor, TokioMultiThreadExecutor};
use buoyant_kernel_engine::executor::TaskExecutor;
use buoyant_kernel_engine::DefaultEngineBuilder;
use delta_kernel::snapshot::{Snapshot, SnapshotRef};
use delta_kernel::Engine;
use object_store::parse_url_opts;
use url::Url;

use crate::api::sharing::SharingQueryHandler;
use crate::api::{RequestContext, SecuredAction};
use crate::models::sharing::v1::{
    GetTableMetadataRequest, GetTableVersionRequest, GetTableVersionResponse, QueryResponse,
};
use crate::{Policy, ResourceRef, Result, TableLocationResolver};

pub use predicate::json_predicate_to_expression;

mod conversion;
mod predicate;

#[async_trait::async_trait]
pub trait KernelEngineFactroy: Send + Sync {
    async fn create(&self, table_root: &Url) -> Result<Arc<dyn Engine>>;
}

pub struct DefaultKernelEngineFactroy<E: TaskExecutor> {
    task_executor: Arc<E>,
    storage_configs: HashMap<(String, String), HashMap<String, String>>,
}

impl<E: TaskExecutor> DefaultKernelEngineFactroy<E> {
    pub fn new(
        task_executor: Arc<E>,
        storage_configs: HashMap<(String, String), HashMap<String, String>>,
    ) -> Self {
        Self {
            task_executor,
            storage_configs,
        }
    }
}

#[async_trait::async_trait]
impl<E: TaskExecutor> KernelEngineFactroy for DefaultKernelEngineFactroy<E> {
    async fn create(&self, table_root: &Url) -> Result<Arc<dyn delta_kernel::Engine>> {
        let storage_config = self
            .storage_configs
            .get(&(
                table_root.scheme().to_string(),
                table_root.host_str().unwrap_or_default().to_string(),
            ))
            .cloned()
            .unwrap_or_default();
        // The kernel no longer builds the object store from a URL; the engine takes it directly.
        let (store, _path) = parse_url_opts(table_root, storage_config)
            .map_err(|e| crate::Error::InvalidTableLocation(e.to_string()))?;
        let engine = DefaultEngineBuilder::new(Arc::new(store))
            .with_task_executor(self.task_executor.clone())
            .build();
        Ok(Arc::new(engine))
    }
}

pub struct KernelQueryHandler {
    engine_factory: Arc<dyn KernelEngineFactroy>,
    location_resolver: Arc<dyn TableLocationResolver>,
    policy: Arc<dyn Policy>,
}

impl KernelQueryHandler {
    /// Create a new instance of [`KernelQueryHandler`].
    pub fn new(
        engine_factory: Arc<dyn KernelEngineFactroy>,
        location_resolver: Arc<dyn TableLocationResolver>,
        policy: Arc<dyn Policy>,
    ) -> Self {
        Self {
            engine_factory,
            location_resolver,
            policy,
        }
    }

    /// Create a new instance of [`KernelQueryHandler`] with a background executor.
    pub fn new_background(
        location_resolver: Arc<dyn TableLocationResolver>,
        storage_configs: HashMap<(String, String), HashMap<String, String>>,
        policy: Arc<dyn Policy>,
    ) -> Arc<Self> {
        let engine_factory = Arc::new(DefaultKernelEngineFactroy::new(
            Arc::new(TokioBackgroundExecutor::new()),
            storage_configs,
        ));
        Arc::new(Self::new(engine_factory, location_resolver, policy))
    }

    /// Create a new instance of [`KernelQueryHandler`] with a multi-threaded executor.
    pub fn new_multi_thread(
        location_resolver: Arc<dyn TableLocationResolver>,
        storage_configs: HashMap<(String, String), HashMap<String, String>>,
        policy: Arc<dyn Policy>,
    ) -> Arc<Self> {
        let engine_factory = Arc::new(DefaultKernelEngineFactroy::new(
            Arc::new(TokioMultiThreadExecutor::new(
                tokio::runtime::Handle::current(),
            )),
            storage_configs,
        ));
        Arc::new(Self::new(engine_factory, location_resolver, policy))
    }

    async fn get_snapshot(&self, table_ref: &ResourceRef) -> Result<SnapshotRef> {
        let location = self.location_resolver.resolve(table_ref).await?;
        let engine = self.engine_factory.create(&location).await?;
        let snapshot = Snapshot::builder_for(location.as_str()).build(engine.as_ref())?;
        Ok(snapshot)
    }
}

#[async_trait::async_trait]
impl SharingQueryHandler for KernelQueryHandler {
    async fn get_table_version(
        &self,
        request: GetTableVersionRequest,
        context: RequestContext,
    ) -> Result<GetTableVersionResponse> {
        self.policy
            .check_required(&request, context.as_ref())
            .await?;
        let res = request.resource();
        // TODO handle optional timestamp
        let snapshot = self.get_snapshot(res.as_ref()).await?;
        let version = snapshot.version();
        Ok(GetTableVersionResponse {
            version: version as i64,
        })
    }

    async fn get_table_metadata(
        &self,
        request: GetTableMetadataRequest,
        context: RequestContext,
    ) -> Result<QueryResponse> {
        self.policy
            .check_required(&request, context.as_ref())
            .await?;
        let res = request.resource();
        let snapshot = self.get_snapshot(res.as_ref()).await?;
        let table_configuration = snapshot.table_configuration();
        Ok([
            table_configuration.metadata().into(),
            table_configuration.protocol().into(),
        ]
        .into())
    }
}
