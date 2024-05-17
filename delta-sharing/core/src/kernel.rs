use std::collections::HashMap;
use std::sync::Arc;

use delta_kernel::engine::default::executor::tokio::{
    TokioBackgroundExecutor, TokioMultiThreadExecutor,
};
use delta_kernel::engine::default::{executor::TaskExecutor, DefaultEngine};
use delta_kernel::{Engine, Table};

use crate::capabilities::ResponseFormat;
use crate::{types as t, ParquetWrappedMetadata, ParquetWrappedProtocol};
use crate::{Result, TableLocationResover, TableQueryHandler, TableRef};

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
    pub fn new(
        engine_factory: Arc<dyn KernelEngineFactroy>,
        location_resolver: Arc<dyn TableLocationResover>,
    ) -> Self {
        Self {
            engine_factory,
            location_resolver,
        }
    }

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
}

#[async_trait::async_trait]
impl TableQueryHandler for KernelQueryHandler {
    async fn get_table_version(
        &self,
        request: t::GetTableVersionRequest,
    ) -> Result<t::GetTableVersionResponse> {
        let location = self
            .location_resolver
            .resolve(&TableRef {
                share: request.share,
                schema: request.schema,
                table: request.table,
            })
            .await?;

        let table = Table::new(location);
        let engine = self.engine_factory.create(&table).await?;
        let snapshot = table.snapshot(engine.as_ref(), None)?;

        let version = snapshot.version();
        Ok(t::GetTableVersionResponse {
            version: version as i64,
        })
    }

    async fn get_table_metadata(
        &self,
        request: t::GetTableMetadataRequest,
    ) -> Result<t::GetTableMetadataResponse> {
        let location = self
            .location_resolver
            .resolve(&TableRef {
                share: request.share,
                schema: request.schema,
                table: request.table,
            })
            .await?;

        let table = Table::new(location);
        let engine = self.engine_factory.create(&table).await?;
        let snapshot = table.snapshot(engine.as_ref(), None)?;

        let protocol = snapshot.protocol();
        let metadata = snapshot.metadata();
        let response_format = request
            .response_format
            .first()
            .and_then(|format| format.parse::<ResponseFormat>().ok())
            .unwrap_or(ResponseFormat::Parquet);

        let response = match response_format {
            ResponseFormat::Parquet => {
                let parquet_protocol_action = t::ParquetWrappedAction {
                    action: Some(t::parquet_wrapped_action::Action::ParquetProtocol(
                        ParquetWrappedProtocol {
                            min_reader_version: protocol.min_reader_version,
                        },
                    )),
                };
                let parquet_metadata_action = t::ParquetWrappedAction {
                    action: Some(t::parquet_wrapped_action::Action::ParquetMetadata(
                        ParquetWrappedMetadata {
                            id: metadata.id.clone(),
                            name: metadata.name.clone(),
                            description: metadata.description.clone(),
                            schema_string: metadata.schema_string.clone(),
                            partition_columns: metadata.partition_columns.clone(),
                            configuration: metadata.configuration.clone(),
                            format: Some(t::ParquetFormat {
                                provider: metadata.format.provider.clone(),
                            }),
                            version: None,
                            size: None,
                            num_files: None,
                        },
                    )),
                };

                let actions = t::ParquetWrappedResponse {
                    actions: vec![parquet_protocol_action, parquet_metadata_action],
                };
                t::get_table_metadata_response::Response::Parquet(actions)
            }
            ResponseFormat::Delta => {
                let delta_protocol_action = t::DeltaWrappedAction {
                    action: Some(t::delta_wrapped_action::Action::DeltaProtocol(
                        t::DeltaWrappedProtocol {
                            delta_protocol: Some(t::DeltaProtocol {
                                min_reader_version: protocol.min_reader_version,
                                min_writer_version: protocol.min_writer_version,
                                reader_features: protocol
                                    .reader_features
                                    .clone()
                                    .unwrap_or_default(),
                                writer_features: protocol
                                    .writer_features
                                    .clone()
                                    .unwrap_or_default(),
                            }),
                        },
                    )),
                };
                let delta_metadata_action = t::DeltaWrappedAction {
                    action: Some(t::delta_wrapped_action::Action::DeltaMetadata(
                        t::DeltaWrappedMetadata {
                            delta_metadata: Some(t::DeltaMetadata {
                                id: metadata.id.clone(),
                                name: metadata.name.clone(),
                                description: metadata.description.clone(),
                                schema_string: metadata.schema_string.clone(),
                                partition_columns: metadata.partition_columns.clone(),
                                configuration: metadata.configuration.clone(),
                                format: Some(t::DeltaFormat {
                                    provider: metadata.format.provider.clone(),
                                    options: metadata.format.options.clone(),
                                }),
                                created_time: None,
                            }),
                            version: None,
                            size: None,
                            num_files: None,
                        },
                    )),
                };

                let actions = t::DeltaWrappedResponse {
                    actions: vec![delta_protocol_action, delta_metadata_action],
                };
                t::get_table_metadata_response::Response::Delta(actions)
            }
        };

        Ok(t::GetTableMetadataResponse {
            version: snapshot.version(),
            response: Some(response),
        })
    }
}
