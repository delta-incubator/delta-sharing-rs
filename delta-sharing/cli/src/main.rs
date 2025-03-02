use chrono::Days;
use clap::{Args, Parser, Subcommand};
use delta_sharing_profiles::{DefaultClaims, DeltaProfileManager, ProfileManager, TokenManager};

use self::client::{handle_client, ClientCommand};
use crate::error::Result;
use crate::server::{handle_rest, ServerArgs};

mod client;
mod config;
mod error;
mod server;

#[derive(Parser)]
#[command(name = "delta-sharing", version, about = "CLI to manage delta.sharing services.", long_about = None)]
struct Cli {
    #[clap(flatten)]
    global_opts: GlobalOpts,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Args)]
struct GlobalOpts {
    /// Server URL
    #[clap(
        long,
        global = true,
        env = "UC_SERVER_URL",
        default_value = "http://localhost:8080"
    )]
    server: String,
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
    Client(ClientCommand),

    #[clap(
        arg_required_else_help = true,
        about = "create a profile for sharing server"
    )]
    Profile(ProfileArgs),

    #[clap(about = "run database migrations")]
    Migrate,
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

    dbg!("CLI");

    match &args.command {
        Commands::Rest(server_args) => handle_rest(server_args).await?,
        Commands::Grpc(_) => todo!("gRPC server not implemented"),
        Commands::Client(client_args) => {
            handle_client(client_args, args.global_opts).await?;
        }
        Commands::Profile(args) => handle_profile(args).await?,
        Commands::Migrate => todo!(),
    };

    Ok(())
}

/// Handle the profile command.
async fn handle_profile(args: &ProfileArgs) -> Result<()> {
    let token_manager = TokenManager::new_from_secret(args.secret.as_bytes(), None);
    let profile_manager = DeltaProfileManager::new(args.endpoint.clone(), 1, token_manager);

    let exp = args
        .validity
        .and_then(|days| chrono::Utc::now().checked_add_days(Days::new(days)));
    let shares = args
        .shares
        .split(',')
        .map(|s| s.trim().to_ascii_lowercase())
        .collect();
    let claims = DefaultClaims {
        sub: args.subject.clone(),
        issued_at: chrono::Utc::now().timestamp(),
        admin: args.admin,
        exp: exp.as_ref().map(|dt| dt.timestamp() as u64),
        shares,
    };
    let profile = profile_manager.issue_profile(&claims, exp).await?;
    std::fs::write("profile.json", serde_json::to_string_pretty(&profile)?)?;
    Ok(())
}
