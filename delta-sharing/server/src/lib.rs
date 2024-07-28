pub mod auth;
pub mod server;

use std::path::Path;
use std::sync::Arc;

use delta_sharing_common::error::{Error, Result};
use delta_sharing_common::policies::ConstantPolicy;
use delta_sharing_common::{DeltaRecipient, InMemoryConfig, InMemoryHandler, KernelQueryHandler};
use tokio::net::TcpListener;
use tokio::signal;
use tower_http::trace::TraceLayer;

use self::auth::{AnonymousAuthenticator, AuthorizationLayer};
use self::server::{get_router, DeltaSharingState};

pub async fn run_server(config: impl AsRef<Path>, host: impl AsRef<str>, port: u16) -> Result<()> {
    let config = std::fs::read_to_string(config)
        .map_err(|_| Error::Generic("malformed config".to_string()))?;
    let config = serde_yml::from_str::<InMemoryConfig>(&config)
        .map_err(|_| Error::Generic("malformed config".to_string()))?;
    let discovery = Arc::new(InMemoryHandler::new(config));
    let state = DeltaSharingState {
        query: KernelQueryHandler::new_multi_thread(discovery.clone(), Default::default()),
        discovery,
        policy: Arc::new(ConstantPolicy::<DeltaRecipient>::default()),
    };

    let listener = TcpListener::bind(format!("{}:{}", host.as_ref(), port))
        .await
        .map_err(|e| Error::Generic(e.to_string()))?;
    let server = get_router(state)
        .layer(AuthorizationLayer::new(AnonymousAuthenticator))
        .layer(TraceLayer::new_for_http());
    axum::serve(listener, server)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .map_err(|e| Error::Generic(e.to_string()))?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

#[cfg(test)]
mod tests {
    use delta_sharing_common::{
        DefaultInMemoryHandler, InMemoryConfig, SchemaConfig, ShareConfig, TableConfig,
    };

    pub(crate) fn test_config() -> InMemoryConfig {
        InMemoryConfig {
            shares: vec![ShareConfig {
                name: "share1".to_string(),
                schema_refs: vec!["schema1".to_string()],
            }],
            schemas: vec![SchemaConfig {
                name: "schema1".to_string(),
                table_refs: vec!["table1".to_string()],
            }],
            tables: vec![TableConfig {
                name: "table1".to_string(),
                location: "file:///tmp".to_string(),
            }],
        }
    }

    pub(crate) fn test_handler() -> DefaultInMemoryHandler {
        DefaultInMemoryHandler::new(test_config())
    }
}
