use axum::body::Body;
use axum::extract::{Extension, State};
use axum::{response::Response, routing::get, Json, Router};
use http::header::CONTENT_TYPE;

use crate::models::v1::*;
use crate::{
    process_resources, DiscoveryHandler, Error, Permission, Policy, Recipient, Result,
    TableQueryHandler,
};

/// Create a new [Router] for the Delta Sharing REST API.
pub fn get_router<T>(state: T) -> Router
where
    T: DiscoveryHandler + Policy + TableQueryHandler + Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/shares", get(list_shares::<T>))
        .route("/shares/{share}", get(get_share::<T>))
        .route("/shares/{share}/schemas", get(list_schemas::<T>))
        .route("/shares/{share}/all-tables", get(list_share_tables::<T>))
        .route(
            "/shares/{share}/schemas/{schema}/tables",
            get(list_schema_tables::<T>),
        )
        .route(
            "/shares/{share}/schemas/{schema}/tables/{table}/version",
            get(get_table_version::<T>),
        )
        .route(
            "/shares/{share}/schemas/{schema}/tables/{table}/metadata",
            get(get_table_metadata::<T>),
        )
        .with_state(state)
}

async fn list_shares<T: DiscoveryHandler + Policy>(
    State(handler): State<T>,
    Extension(recipient): Extension<Recipient>,
    request: ListSharesRequest,
) -> Result<Json<ListSharesResponse>> {
    let mut shares = handler.list_shares(request, &recipient).await?;
    process_resources(&handler, &recipient, &Permission::Read, &mut shares.items).await?;
    Ok(Json(shares))
}

async fn get_share<T: DiscoveryHandler + Policy>(
    State(handler): State<T>,
    Extension(recipient): Extension<Recipient>,
    request: GetShareRequest,
) -> Result<Json<Share>> {
    handler.check_required(&request, &recipient).await?;
    Ok(Json(handler.get_share(request).await?))
}

async fn list_schemas<T: DiscoveryHandler + Policy>(
    State(handler): State<T>,
    Extension(recipient): Extension<Recipient>,
    request: ListSchemasRequest,
) -> Result<Json<ListSchemasResponse>> {
    handler.check_required(&request, &recipient).await?;
    Ok(Json(handler.list_schemas(request).await?))
}

async fn list_share_tables<T: DiscoveryHandler + Policy>(
    State(handler): State<T>,
    Extension(recipient): Extension<Recipient>,
    request: ListShareTablesRequest,
) -> Result<Json<ListShareTablesResponse>> {
    handler.check_required(&request, &recipient).await?;
    Ok(Json(handler.list_share_tables(request).await?))
}

async fn list_schema_tables<T: DiscoveryHandler + Policy>(
    State(handler): State<T>,
    Extension(recipient): Extension<Recipient>,
    request: ListSchemaTablesRequest,
) -> Result<Json<ListSchemaTablesResponse>> {
    handler.check_required(&request, &recipient).await?;
    Ok(Json(handler.list_schema_tables(request).await?))
}

async fn get_table_version<T: Policy + TableQueryHandler>(
    State(handler): State<T>,
    Extension(recipient): Extension<Recipient>,
    request: GetTableVersionRequest,
) -> Result<Response<Body>> {
    handler.check_required(&request, &recipient).await?;
    let result = handler.get_table_version(request).await?;
    let response = Response::builder()
        .header("Delta-Table-Version", result.version)
        .body(Body::empty())
        .map_err(|e| Error::generic(e.to_string()))?;
    Ok(response)
}

async fn get_table_metadata<T: Policy + TableQueryHandler>(
    State(handler): State<T>,
    Extension(recipient): Extension<Recipient>,
    request: GetTableMetadataRequest,
) -> Result<Response<Body>> {
    handler.check_required(&request, &recipient).await?;
    let result = handler.get_table_metadata(request).await?;
    let response = Response::builder()
        .header(CONTENT_TYPE, "application/x-ndjson; charset=utf-8")
        .body(Body::from(query_response_to_ndjson(result)?))
        .map_err(|e| Error::generic(e.to_string()))?;
    Ok(response)
}

fn query_response_to_ndjson(response: impl IntoIterator<Item = Result<String>>) -> Result<String> {
    Ok(response
        .into_iter()
        .collect::<Result<Vec<String>>>()?
        .join("\n"))
}
