use delta_sharing_common::rest::get_sharing_router;
use delta_sharing_common::{DeltaSharingHandler, Error, Result};
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

    let server = get_sharing_router(handler)
        .merge(swagger_ui_dist::generate_routes(api_def))
        .layer(AuthenticationLayer::new(AnonymousAuthenticator))
        .layer(
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

    axum::serve(listener, server)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .map_err(|e| Error::Generic(e.to_string()))?;

    Ok(())
}
