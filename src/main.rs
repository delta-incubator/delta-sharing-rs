use anyhow::Context;
use anyhow::Result;
use kotosiro_sharing::config::Config;
use kotosiro_sharing::logging;
use kotosiro_sharing::server::Server;
use tracing::debug;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let app = clap::Command::new("kotosiro-sharing")
        .author("Shingo OKAWA <shingo.okawa.g.h.c@gmail.com>")
        .version(kotosiro_sharing::VERSION)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .arg(
            clap::Arg::new("config")
                .long("config")
                .short('c')
                .help("Provide a specific config file"),
        )
        .subcommand(
            clap::Command::new("server")
                .about("Launch the server process")
                .after_help("The server implements Delta Sharing REST protocol."),
        );
    let args = app.get_matches();
    let conf = args.get_one::<String>("config").map(AsRef::as_ref);
    let conf = Config::load(conf)?;
    logging::setup(&conf);
    debug!(
        db_url = &conf.db_url,
        server_addr = &conf.server_addr,
        server_bind = &conf.server_bind,
        use_json_log = &conf.use_json_log
    );
    match args.subcommand().expect("subcommand is required") {
        ("server", _args) => {
            debug!("server is called");
            let server = Server::new(conf).await.context("failed to create server")?;
            server.start().await.context("failed to start server")
        }
        _ => unreachable!("clap should have already checked the subcommands"),
    }
}
