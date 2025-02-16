use std::path::Path;
use std::sync::Arc;

use chrono::Days;
use clap::{Parser, Subcommand};
use delta_sharing_common::{
    ConstantPolicy, DeltaSharingHandler, InMemoryConfig, InMemoryHandler, KernelQueryHandler,
};
use delta_sharing_profiles::{DefaultClaims, DeltaProfileManager, ProfileManager, TokenManager};
use delta_sharing_server::{run_grpc_server, run_rest_server};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::error::{Error, Result};

mod config;
mod error;

#[derive(Parser)]
#[command(name = "delta-sharing", version, about = "CLI to manage delta.sharing services.", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(about = "start a sharing server (REST)")]
    Rest(ServerArgs),

    #[clap(about = "start a sharing server (gRPC)")]
    Grpc(ServerArgs),

    #[clap(
        arg_required_else_help = true,
        about = "execute requests against a sharing server"
    )]
    Client(ClientArgs),

    #[clap(
        arg_required_else_help = true,
        about = "create a profile for sharing server"
    )]
    Profile(ProfileArgs),

    #[clap(about = "run database migrations")]
    Migrate,
}

#[derive(Parser)]
struct ServerArgs {
    #[clap(long, default_value = "0.0.0.0")]
    host: String,

    #[clap(long, short, default_value_t = 8080)]
    port: u16,

    #[arg(short, long, default_value = "config.yaml")]
    config: String,
}

#[derive(Parser)]
struct ClientArgs {
    #[clap(help = "Sets the server address")]
    endpoint: String,
}

#[derive(Parser)]
struct ProfileArgs {
    #[clap(long, help = "secret used to encode the profile token")]
    secret: String,

    #[clap(long, short, help = "server endpoint")]
    endpoint: String,

    #[clap(
        long,
        help = "subject the profile is issued to - often an email address"
    )]
    subject: String,

    #[clap(long, short, help = "validity period in days")]
    validity: Option<u64>,

    #[clap(
        long,
        help = "comma separated list of shares the profile has access to"
    )]
    shares: String,

    #[clap(long, help = "admin flag")]
    admin: Option<bool>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse the command-line arguments
    let args = Cli::parse();

    match args.command {
        Commands::Rest(server_args) => handle_rest(server_args).await?,
        Commands::Grpc(server_args) => handle_grpc(server_args).await?,
        Commands::Client(client_args) => {
            // Access the client arguments
            let endpoint = client_args.endpoint;
            // Start the client logic
            println!("Connecting to server at {}", endpoint);
            // Your client logic goes here
        }
        Commands::Profile(args) => handle_profile(args).await?,
        Commands::Migrate => todo!(),
    };

    Ok(())
}

fn get_handler(config: impl AsRef<Path>) -> Result<DeltaSharingHandler> {
    let config = std::fs::read_to_string(config)
        .map_err(|_| Error::Generic("malformed config".to_string()))?;
    let config = serde_yml::from_str::<InMemoryConfig>(&config)
        .map_err(|_| Error::Generic("malformed config".to_string()))?;

    let discovery = Arc::new(InMemoryHandler::new(config));
    let handler = DeltaSharingHandler {
        query: KernelQueryHandler::new_multi_thread(discovery.clone(), Default::default()),
        discovery,
        policy: Arc::new(ConstantPolicy::default()),
    };
    Ok(handler)
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

/// Handle the rest server command.
///
/// This function starts a delta-sharing server using the REST protocol.
async fn handle_rest(args: ServerArgs) -> Result<()> {
    init_tracing();

    let handler = get_handler(args.config)?;

    run_rest_server(args.host, args.port, handler)
        .await
        .map_err(|_| Error::Generic("Server failed".to_string()))
}

/// Handle the server command.
///
/// This function starts a delta-sharing server using the gRPC protocol.
async fn handle_grpc(args: ServerArgs) -> Result<()> {
    init_tracing();

    let handler = get_handler(args.config)?;

    run_grpc_server(args.host, args.port, handler)
        .await
        .map_err(|_| Error::Generic("Server failed".to_string()))
}

/// Handle the profile command.
async fn handle_profile(args: ProfileArgs) -> Result<()> {
    let token_manager = TokenManager::new_from_secret(args.secret.as_bytes(), None);
    let profile_manager = DeltaProfileManager::new(args.endpoint, 1, token_manager);

    let exp = args
        .validity
        .and_then(|days| chrono::Utc::now().checked_add_days(Days::new(days)));
    let shares = args
        .shares
        .split(',')
        .map(|s| s.trim().to_ascii_lowercase())
        .collect();
    let claims = DefaultClaims {
        sub: args.subject,
        issued_at: chrono::Utc::now().timestamp(),
        admin: args.admin,
        exp: exp.as_ref().map(|dt| dt.timestamp() as u64),
        shares,
    };
    let profile = profile_manager.issue_profile(&claims, exp).await?;
    std::fs::write("profile.json", serde_json::to_string_pretty(&profile)?)?;
    Ok(())
}
