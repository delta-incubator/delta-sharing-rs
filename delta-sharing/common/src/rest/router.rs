use axum::body::Body;
use axum::extract::{Extension, State};
use axum::{response::Response, routing::get, Json, Router};
use http::header::CONTENT_TYPE;

use crate::models::sharing::v1::*;
use crate::{DiscoveryManager, Error, Recipient, Result, TableQueryManager};

/// Create a new [Router] for the Delta Sharing REST API.
pub fn get_router<T: DiscoveryManager + TableQueryManager + Clone>(state: T) -> Router {
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

async fn list_shares<T: DiscoveryManager>(
    State(handler): State<T>,
    Extension(recipient): Extension<Recipient>,
    request: ListSharesRequest,
) -> Result<Json<ListSharesResponse>> {
    Ok(Json(handler.list_shares(request, &recipient).await?))
}

async fn get_share<T: DiscoveryManager>(
    State(handler): State<T>,
    Extension(recipient): Extension<Recipient>,
    request: GetShareRequest,
) -> Result<Json<Share>> {
    Ok(Json(handler.get_share(request, &recipient).await?))
}

async fn list_schemas<T: DiscoveryManager>(
    State(handler): State<T>,
    Extension(recipient): Extension<Recipient>,
    request: ListSharingSchemasRequest,
) -> Result<Json<ListSharingSchemasResponse>> {
    Ok(Json(handler.list_schemas(request, &recipient).await?))
}

async fn list_share_tables<T: DiscoveryManager>(
    State(handler): State<T>,
    Extension(recipient): Extension<Recipient>,
    request: ListShareTablesRequest,
) -> Result<Json<ListShareTablesResponse>> {
    Ok(Json(handler.list_share_tables(request, &recipient).await?))
}

async fn list_schema_tables<T: DiscoveryManager>(
    State(handler): State<T>,
    Extension(recipient): Extension<Recipient>,
    request: ListSchemaTablesRequest,
) -> Result<Json<ListSchemaTablesResponse>> {
    Ok(Json(handler.list_schema_tables(request, &recipient).await?))
}

async fn get_table_version<T: TableQueryManager>(
    State(handler): State<T>,
    Extension(recipient): Extension<Recipient>,
    request: GetTableVersionRequest,
) -> Result<Response<Body>> {
    let result = handler.get_table_version(request, &recipient).await?;
    let response = Response::builder()
        .header("Delta-Table-Version", result.version)
        .body(Body::empty())
        .map_err(|e| Error::generic(e.to_string()))?;
    Ok(response)
}

async fn get_table_metadata<T: TableQueryManager>(
    State(handler): State<T>,
    Extension(recipient): Extension<Recipient>,
    request: GetTableMetadataRequest,
) -> Result<Response<Body>> {
    let result = handler.get_table_metadata(request, &recipient).await?;
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
