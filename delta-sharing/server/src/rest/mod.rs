use delta_sharing_common::rest::{get_sharing_repo_router, get_sharing_router};
use delta_sharing_common::{
    DeltaSharingHandler, Error, Policy, Result, SharingRepository, TableQueryHandler,
};
use swagger_ui_dist::{ApiDefinition, OpenApiSource};
use tokio::net::TcpListener;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tower_http::LatencyUnit;
use tracing::Level;

use crate::rest::auth::{AnonymousAuthenticator, AuthenticationLayer};
use crate::shutdown::shutdown_signal;

mod auth;
#[cfg(test)]
mod tests;

pub async fn run_server(
    host: impl AsRef<str>,
    port: u16,
    handler: DeltaSharingHandler,
) -> Result<()> {
    let api_def = ApiDefinition {
        uri_prefix: "/api",
        api_definition: OpenApiSource::Inline(include_str!("../../openapi.yaml")),
        title: Some("My Super Duper API"),
    };
    let server =
        get_sharing_router(handler).layer(AuthenticationLayer::new(AnonymousAuthenticator));
    run(server, host, port, api_def).await
}

pub async fn run_server_full<T>(host: impl AsRef<str>, port: u16, handler: T) -> Result<()>
where
    T: SharingRepository + TableQueryHandler + Policy + Clone + Send + Sync + 'static,
{
    let api_def = ApiDefinition {
        uri_prefix: "/api",
        api_definition: OpenApiSource::Inline(include_str!("../../openapi.yaml")),
        title: Some("My Super Duper API"),
    };
    let router = get_sharing_router(handler.clone()).merge(get_sharing_repo_router(handler));
    let server = router.layer(AuthenticationLayer::new(AnonymousAuthenticator));
    run(server, host, port, api_def).await
}

async fn run<S: Into<String> + Clone>(
    router: axum::Router,
    host: impl AsRef<str>,
    port: u16,
    api: ApiDefinition<S>,
) -> Result<()> {
    let router = router.merge(swagger_ui_dist::generate_routes(api)).layer(
        TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::new().include_headers(true))
            .on_request(DefaultOnRequest::new().level(Level::INFO))
            .on_response(
                DefaultOnResponse::new()
                    .level(Level::INFO)
                    .latency_unit(LatencyUnit::Micros),
            ),
    );
    let listener = TcpListener::bind(format!("{}:{}", host.as_ref(), port))
        .await
        .map_err(|e| Error::Generic(e.to_string()))?;
    tracing::info!("Listning on: {}", listener.local_addr().unwrap());
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .map_err(|e| Error::Generic(e.to_string()))?;

    Ok(())
}
