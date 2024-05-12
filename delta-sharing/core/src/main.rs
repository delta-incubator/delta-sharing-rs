use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    subcommand: Subcommand,
}

#[derive(Parser)]
enum Subcommand {
    #[clap(arg_required_else_help = true, about = "start a sharing server")]
    Server(ServerArgs),

    #[clap(
        arg_required_else_help = true,
        about = "execute requests against a sharing server"
    )]
    Client(ClientArgs),
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

fn main() {
    // Parse the command-line arguments
    let args = Cli::parse();

    match args.subcommand {
        Subcommand::Server(server_args) => {
            // Access the server arguments
            let port = server_args.port;
            // Start the server logic
            println!("Starting server on port {}", port);
            // Your server logic goes here
        }
        Subcommand::Client(client_args) => {
            // Access the client arguments
            let endpoint = client_args.endpoint;
            // Start the client logic
            println!("Connecting to server at {}", endpoint);
            // Your client logic goes here
        }
    }
}
