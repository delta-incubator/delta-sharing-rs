use std::collections::HashMap;
use std::sync::Arc;

use delta_kernel::engine::default::executor::tokio::{
    TokioBackgroundExecutor, TokioMultiThreadExecutor,
};
use delta_kernel::engine::default::{executor::TaskExecutor, DefaultEngine};
use delta_kernel::snapshot::Snapshot;
use delta_kernel::{Engine, Table};

use crate::models::v1::{
    GetTableMetadataRequest, GetTableVersionRequest, GetTableVersionResponse, QueryResponse,
};
use crate::models::TableRef;
use crate::{Result, TableLocationResover, TableQueryHandler};

mod conversion;

#[async_trait::async_trait]
pub trait KernelEngineFactroy: Send + Sync {
    async fn create(&self, table: &Table) -> Result<Arc<dyn Engine>>;
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
    async fn create(&self, table: &Table) -> Result<Arc<dyn delta_kernel::Engine>> {
        let storage_config = self
            .storage_configs
            .get(&(
                table.location().scheme().to_string(),
                table.location().host_str().unwrap_or_default().to_string(),
            ))
            .cloned()
            .unwrap_or_default();
        let engine =
            DefaultEngine::try_new(table.location(), storage_config, self.task_executor.clone())?;
        Ok(Arc::new(engine))
    }
}

pub struct KernelQueryHandler {
    engine_factory: Arc<dyn KernelEngineFactroy>,
    location_resolver: Arc<dyn TableLocationResover>,
}

impl KernelQueryHandler {
    /// Create a new instance of [`KernelQueryHandler`].
    pub fn new(
        engine_factory: Arc<dyn KernelEngineFactroy>,
        location_resolver: Arc<dyn TableLocationResover>,
    ) -> Self {
        Self {
            engine_factory,
            location_resolver,
        }
    }

    /// Create a new instance of [`KernelQueryHandler`] with a background executor.
    pub fn new_background(
        location_resolver: Arc<dyn TableLocationResover>,
        storage_configs: HashMap<(String, String), HashMap<String, String>>,
    ) -> Arc<Self> {
        let engine_factory = Arc::new(DefaultKernelEngineFactroy::new(
            Arc::new(TokioBackgroundExecutor::new()),
            storage_configs,
        ));
        Arc::new(Self::new(engine_factory, location_resolver))
    }

    /// Create a new instance of [`KernelQueryHandler`] with a multi-threaded executor.
    pub fn new_multi_thread(
        location_resolver: Arc<dyn TableLocationResover>,
        storage_configs: HashMap<(String, String), HashMap<String, String>>,
    ) -> Arc<Self> {
        let engine_factory = Arc::new(DefaultKernelEngineFactroy::new(
            Arc::new(TokioMultiThreadExecutor::new(
                tokio::runtime::Handle::current(),
            )),
            storage_configs,
        ));
        Arc::new(Self::new(engine_factory, location_resolver))
    }

    async fn get_snapshot(&self, table_ref: &TableRef) -> Result<Snapshot> {
        let location = self.location_resolver.resolve(table_ref).await?;
        let table = Table::new(location);
        let engine = self.engine_factory.create(&table).await?;
        let snapshot = table.snapshot(engine.as_ref(), None)?;
        Ok(snapshot)
    }
}

#[async_trait::async_trait]
impl TableQueryHandler for KernelQueryHandler {
    async fn get_table_version(
        &self,
        request: GetTableVersionRequest,
    ) -> Result<GetTableVersionResponse> {
        let snapshot = self
            .get_snapshot(&TableRef {
                share: request.share,
                schema: request.schema,
                table: request.name,
            })
            .await?;

        let version = snapshot.version();
        Ok(GetTableVersionResponse {
            version: version as i64,
        })
    }

    async fn get_table_metadata(&self, request: GetTableMetadataRequest) -> Result<QueryResponse> {
        let snapshot = self
            .get_snapshot(&TableRef {
                share: request.share,
                schema: request.schema,
                table: request.name,
            })
            .await?;
        Ok([snapshot.metadata().into(), snapshot.protocol().into()].into())
    }
}
