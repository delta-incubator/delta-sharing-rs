use anyhow::Context;
use anyhow::Result;
use kotosiro_sharing::config;
use kotosiro_sharing::logging;
use kotosiro_sharing::server::Server;

#[tokio::main]
async fn main() -> Result<()> {
    let app = clap::Command::new("kotosiro-sharing")
        .author("Shingo OKAWA <shingo.okawa.g.h.c@gmail.com>")
        .version(kotosiro_sharing::VERSION)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            clap::Command::new("server")
                .about("Launch the server process")
                .after_help("The server implements Delta Sharing REST protocol."),
        );
    logging::setup();
    tracing::debug!(
        db_url = config::fetch::<String>("db_url"),
        server_addr = config::fetch::<String>("server_addr"),
        server_bind = config::fetch::<String>("server_bind"),
        jwt_secret = config::fetch::<String>("jwt_secret"),
        admin_name = config::fetch::<String>("admin_name"),
        admin_email = config::fetch::<String>("admin_email"),
        admin_password = config::fetch::<String>("admin_password"),
        admin_namespace = config::fetch::<String>("admin_namespace"),
        admin_ttl = config::fetch::<i64>("admin_ttl"),
        gcp_sa_private_key = config::fetch::<String>("gcp_sa_private_key"),
        aws_credentials = config::fetch::<String>("aws_credentials"),
        aws_profile = config::fetch::<String>("aws_profile"),
        use_json_log = config::fetch::<bool>("use_json_log"),
        log_filter = config::fetch::<String>("log_filter"),
    );
    let args = app.get_matches();
    match args.subcommand().expect("subcommand is required") {
        ("server", _args) => {
            tracing::info!("kotosiro sharing server is starting");
            let server = Server::new().await.context("failed to create server")?;
            server.start().await.context("failed to start server")
        }
        _ => unreachable!("clap should have already checked the subcommands"),
    }
}
