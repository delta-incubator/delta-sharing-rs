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
    let args = app.get_matches();
    match args.subcommand().expect("subcommand is required") {
        ("server", _args) => {
            logging::setup();
            let aws_shared_credentials_file = format!(
                "{}",
                shellexpand::tilde(
                    std::env::var("AWS_SHARED_CREDENTIALS_FILE")
                        .ok()
                        .unwrap_or("~/.aws/credentials".into())
                        .as_str()
                )
            );
            let google_applicayion_credentials = format!(
                "{}",
                shellexpand::tilde(
                    std::env::var("GOOGLE_APPLICATION_CREDENTIALS")
                        .ok()
                        .unwrap_or("~/.gcp/service-account-file.json".into())
                        .as_str()
                )
            );
            tracing::info!("kotosiro sharing server is starting");
            tracing::debug!(
                AWS_SHARED_CREDENTIALS_FILE = aws_shared_credentials_file,
                GOOGLE_APPLICATION_CREDENTIALS = google_applicayion_credentials,
                db_url = config::fetch::<String>("db_url"),
                server_addr = config::fetch::<String>("server_addr"),
                server_bind = config::fetch::<String>("server_bind"),
                jwt_secret = config::fetch::<String>("jwt_secret"),
                admin_name = config::fetch::<String>("admin_name"),
                admin_email = config::fetch::<String>("admin_email"),
                admin_password = config::fetch::<String>("admin_password"),
                admin_namespace = config::fetch::<String>("admin_namespace"),
                admin_ttl = config::fetch::<i64>("admin_ttl"),
                signed_url_ttl = config::fetch::<i64>("signed_url_ttl"),
                aws_profile = config::fetch::<String>("aws_profile"),
                aws_region = config::fetch::<String>("aws_region"),
                use_json_log = config::fetch::<bool>("use_json_log"),
                log_filter = config::fetch::<String>("log_filter"),
            );
            let server = Server::new().await.context("failed to create server")?;
            server.start().await.context("failed to start server")
        }
        _ => unreachable!("clap should have already checked the subcommands"),
    }
}
