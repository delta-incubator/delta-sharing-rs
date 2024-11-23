use std::path::Path;
use std::sync::Arc;

use tonic::transport::Server;

use crate::error::{Error, Result};
use crate::models::v1::delta_sharing_service_server::DeltaSharingServiceServer;
use crate::policies::ConstantPolicy;
use crate::{DeltaSharingHandler, InMemoryConfig, InMemoryHandler, KernelQueryHandler};

// TODO(roeap)
// - make auth configurable
// - make log level configurable

pub async fn run_grpc_server(
    config: impl AsRef<Path>,
    host: impl AsRef<str>,
    port: u16,
) -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let config = std::fs::read_to_string(config)
        .map_err(|_| Error::Generic("malformed config".to_string()))?;
    let config = serde_yml::from_str::<InMemoryConfig>(&config)
        .map_err(|_| Error::Generic("malformed config".to_string()))?;

    let discovery = Arc::new(InMemoryHandler::new(config));
    let state = DeltaSharingHandler {
        query: KernelQueryHandler::new_multi_thread(discovery.clone(), Default::default()),
        discovery,
        policy: Arc::new(ConstantPolicy::default()),
    };

    // TODO: remove unwrap
    let addr = format!("{}:{}", host.as_ref(), port)
        .parse()
        .map_err(|_| Error::generic("Invalid address."))?;

    tracing::info!("Listning on: {addr}");

    Server::builder()
        .trace_fn(|_| tracing::info_span!("delta_sharing_server"))
        .add_service(DeltaSharingServiceServer::new(state))
        .serve(addr)
        .await
        .map_err(|e| Error::Generic(e.to_string()))?;

    Ok(())
}
