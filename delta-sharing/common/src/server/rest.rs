use std::path::Path;
use std::sync::Arc;

use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

use super::shutdown_signal;
use crate::error::{Error, Result};
use crate::policies::ConstantPolicy;
use crate::rest::auth::{AnonymousAuthenticator, AuthenticationLayer};
use crate::rest::get_rest_router;
use crate::{DeltaSharingHandler, InMemoryConfig, InMemoryHandler, KernelQueryHandler};

pub async fn run_rest_server(
    config: impl AsRef<Path>,
    host: impl AsRef<str>,
    port: u16,
) -> Result<()> {
    let config = std::fs::read_to_string(config)
        .map_err(|_| Error::Generic("malformed config".to_string()))?;
    let config = serde_yml::from_str::<InMemoryConfig>(&config)
        .map_err(|_| Error::Generic("malformed config".to_string()))?;

    let discovery = Arc::new(InMemoryHandler::new(config));
    let state = DeltaSharingHandler {
        query: KernelQueryHandler::new_multi_thread(discovery.clone(), Default::default()),
        discovery,
        policy: Arc::new(ConstantPolicy::default()),
    };

    let listener = TcpListener::bind(format!("{}:{}", host.as_ref(), port))
        .await
        .map_err(|e| Error::Generic(e.to_string()))?;
    let server = get_rest_router(state)
        .layer(AuthenticationLayer::new(AnonymousAuthenticator))
        .layer(TraceLayer::new_for_http());

    axum::serve(listener, server)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .map_err(|e| Error::Generic(e.to_string()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_run_rest_server() {
        let config = "config.yaml";
    }
}
