use delta_sharing_common::api::catalogs::CatalogHandler;
use delta_sharing_common::api::credentials::CredentialsHandler;
use delta_sharing_common::api::external_locations::ExternalLocationsHandler;
use delta_sharing_common::api::recipients::RecipientsHandler;
use delta_sharing_common::api::schemas::SchemasHandler;
use delta_sharing_common::api::shares::SharesHandler;
use delta_sharing_common::api::sharing::{SharingDiscoveryHandler, SharingQueryHandler};
use delta_sharing_common::rest::{
    get_catalog_router, get_credentials_router, get_external_locations_router,
    get_recipients_router, get_schemas_router, get_shares_router, get_sharing_router,
    AuthenticationLayer, Authenticator,
};
use delta_sharing_common::{Error, Result};
use swagger_ui_dist::{ApiDefinition, OpenApiSource};
use tokio::net::TcpListener;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tower_http::LatencyUnit;
use tracing::Level;

use crate::shutdown::shutdown_signal;

pub async fn run_server_full<T, A>(
    host: impl AsRef<str>,
    port: u16,
    handler: T,
    authenticator: A,
) -> Result<()>
where
    T: CatalogHandler
        + CredentialsHandler
        + SharingDiscoveryHandler
        + SharingQueryHandler
        + SharesHandler
        + SchemasHandler
        + ExternalLocationsHandler
        + RecipientsHandler
        + Clone,
    A: Authenticator + Clone,
{
    let api_def = ApiDefinition {
        uri_prefix: "/api",
        api_definition: OpenApiSource::Inline(include_str!("../../openapi.yaml")),
        title: Some("Unity Catalog API"),
    };
    let router = get_catalog_router(handler.clone())
        .merge(get_schemas_router(handler.clone()))
        .merge(get_credentials_router(handler.clone()))
        .merge(get_external_locations_router(handler.clone()))
        .merge(get_recipients_router(handler.clone()))
        .merge(get_shares_router(handler.clone()));
    let server = router.layer(AuthenticationLayer::new(authenticator));
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
