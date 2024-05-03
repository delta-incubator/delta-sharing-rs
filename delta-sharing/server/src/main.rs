use std::sync::Arc;

use clap::Parser;
use delta_sharing_core::handlers::{Config, InMemoryHandler, VoidRecipientHandler};
use tokio::net::TcpListener;

use self::server::{get_router, DeltaSharingState};

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let config = std::fs::read_to_string(args.config)?;
    let config = serde_yml::from_str::<Config>(&config)?;
    let state = DeltaSharingState {
        discovery: Arc::new(InMemoryHandler::new(config)),
        auth: Arc::new(VoidRecipientHandler {}),
    };

    let listener = TcpListener::bind(format!("{}:{}", args.host, args.port)).await?;
    axum::serve(listener, get_router(state)).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use delta_sharing_core::handlers::{
        Config, InMemoryHandler, SchemaConfig, ShareConfig, TableConfig,
    };

    pub(crate) fn test_config() -> Config {
        Config {
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

    pub(crate) fn test_handler() -> InMemoryHandler {
        InMemoryHandler::new(test_config())
    }
}
