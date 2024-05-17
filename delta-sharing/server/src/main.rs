use std::sync::Arc;

use clap::Parser;
use delta_sharing_core::policies::ConstantPolicy;
use delta_sharing_core::{DeltaRecipient, InMemoryConfig, InMemoryHandler, KernelQueryHandler};
use tokio::net::TcpListener;
use tokio::signal;
use tower_http::trace::TraceLayer;

use self::auth::{AnonymousAuthenticator, AuthorizationLayer};
use self::server::{get_router, DeltaSharingState};

mod auth;
mod error;
pub mod extractors;
mod server;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(long, default_value = "0.0.0.0")]
    host: String,

    #[arg(short, long, default_value_t = 8000)]
    port: u16,

    #[arg(short, long, default_value = "config.yaml")]
    config: String,
}

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let config = std::fs::read_to_string(args.config)?;
    let config = serde_yml::from_str::<InMemoryConfig>(&config)?;
    let discovery = Arc::new(InMemoryHandler::new(config));
    let state = DeltaSharingState {
        query: KernelQueryHandler::new_multi_thread(discovery.clone(), Default::default()),
        discovery,
        policy: Arc::new(ConstantPolicy::<DeltaRecipient>::default()),
    };

    let listener = TcpListener::bind(format!("{}:{}", args.host, args.port)).await?;
    let server = get_router(state)
        .layer(AuthorizationLayer::new(AnonymousAuthenticator))
        .layer(TraceLayer::new_for_http());
    axum::serve(listener, server)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

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
    use delta_sharing_core::{
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
