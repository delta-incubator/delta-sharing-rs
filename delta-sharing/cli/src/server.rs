use std::sync::{Arc, LazyLock};

use clap::Parser;
use delta_sharing_common::{
    memory::InMemoryResourceStore, rest::AnonymousAuthenticator, ConstantPolicy,
    KernelQueryHandler, ServerHandler,
};
use delta_sharing_postgres::GraphStore;
use delta_sharing_server::run_rest_server_full;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::error::{Error, Result};

#[derive(Parser)]
pub struct ServerArgs {
    #[clap(long, default_value = "0.0.0.0")]
    host: String,

    #[clap(long, short, default_value_t = 8080)]
    port: u16,

    #[arg(short, long, default_value = "config.yaml")]
    config: String,

    #[clap(long, help = "use database", default_value_t = false)]
    use_db: bool,
}

async fn get_db_handler() -> Result<ServerHandler> {
    let db_url = std::env::var("DATABASE_URL")
        .map_err(|_| Error::Generic("missing DATABASE_URL".to_string()))?;
    let store = Arc::new(GraphStore::connect(&db_url).await.unwrap());
    let policy = Arc::new(ConstantPolicy::default());
    store.migrate().await.unwrap();
    let handler = ServerHandler {
        query: KernelQueryHandler::new_multi_thread(
            store.clone(),
            Default::default(),
            policy.clone(),
        ),
        store,
        policy,
    };
    Ok(handler)
}

fn get_memory_handler() -> ServerHandler {
    let store = Arc::new(InMemoryResourceStore::new());
    let policy = Arc::new(ConstantPolicy::default());
    ServerHandler {
        query: KernelQueryHandler::new_multi_thread(
            store.clone(),
            Default::default(),
            policy.clone(),
        ),
        store,
        policy,
    }
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
pub async fn handle_rest(args: ServerArgs) -> Result<()> {
    init_tracing();

    println!("{}", WELCOME.as_str());

    if args.use_db {
        let handler = get_db_handler().await?;
        run_rest_server_full(args.host, args.port, handler, AnonymousAuthenticator)
            .await
            .map_err(|_| Error::Generic("Server failed".to_string()))
    } else {
        let handler = get_memory_handler();
        run_rest_server_full(args.host, args.port, handler, AnonymousAuthenticator)
            .await
            .map_err(|_| Error::Generic("Server failed".to_string()))
    }
}

static WELCOME: LazyLock<String> = LazyLock::new(|| {
    format!(
        r#"
     _____       _ _           _____ _                _                        _____   _____
    |  __ \     | | |         / ____| |              (_)                      |  __ \ / ____|
    | |  | | ___| | |_ __ _  | (___ | |__   __ _ _ __ _ _ __   __ _   ______  | |__) | (___
    | |  | |/ _ \ | __/ _` |  \___ \| '_ \ / _` | '__| | '_ \ / _` | |______| |  _  / \___ \
    | |__| |  __/ | || (_| |  ____) | | | | (_| | |  | | | | | (_| |          | | \ \ ____) |
    |_____/ \___|_|\__\__,_| |_____/|_| |_|\__,_|_|  |_|_| |_|\__, |          |_|  \_\_____/
                                                               __/ |
    version: {}                                            |___/
    "#,
        env!("CARGO_PKG_VERSION")
    )
});
