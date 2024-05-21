use chrono::Days;
use clap::{Parser, Subcommand};
use delta_sharing_common::{DefaultClaims, DeltaProfileManager, ProfileManager, TokenManager};

#[derive(Parser)]
#[command(name = "delta-sharing", version, about = "CLI to manage delta.sharing services.", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(arg_required_else_help = true, about = "start a sharing server")]
    Server(ServerArgs),

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
}

#[derive(Parser)]
struct ServerArgs {
    #[clap(long, default_value = "0.0.0.0")]
    host: String,

    #[clap(long, short, default_value = "8080")]
    port: u16,
}

#[derive(Parser)]
struct ClientArgs {
    #[clap(help = "Sets the server address")]
    endpoint: String,
}

#[derive(Parser)]
struct ProfileArgs {
    #[clap(long, short, help = "secret used to encode the profile token")]
    secret: String,

    #[clap(long, short, help = "server endpoint")]
    endpoint: String,

    #[clap(long, short, help = "file containing profile claims")]
    claims: String,

    #[clap(long, short, help = "validity period in days")]
    validity: Option<u64>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse the command-line arguments
    let args = Cli::parse();

    match args.command {
        Commands::Server(server_args) => {
            // Access the server arguments
            let port = server_args.port;
            // Start the server logic
            println!("Starting server on port {}", port);
            // Your server logic goes here
        }
        Commands::Client(client_args) => {
            // Access the client arguments
            let endpoint = client_args.endpoint;
            // Start the client logic
            println!("Connecting to server at {}", endpoint);
            // Your client logic goes here
        }
        Commands::Profile(args) => {
            let token_manager = TokenManager::new_from_secret(args.secret.as_bytes(), None);
            let profile_manager = DeltaProfileManager::new(args.endpoint, 1, token_manager);
            // Access the profile arguments
            let exp = chrono::Utc::now()
                .checked_add_days(Days::new(args.validity.unwrap_or(30)))
                .unwrap();
            let claims = DefaultClaims {
                sub: args.claims,
                exp: Some(exp.timestamp_millis() as u64),
                issued_at: chrono::Utc::now().timestamp(),
                shares: vec![],
                admin: None,
            };
            let profile = profile_manager.issue_profile(&claims, Some(exp)).await?;
            std::fs::write("profile.json", serde_json::to_string_pretty(&profile)?)?;
        }
    };

    Ok(())
}
