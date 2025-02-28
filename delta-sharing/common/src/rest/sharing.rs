use axum::body::Body;
use axum::extract::{Extension, State};
use axum::response::Response;
use axum::routing::{get, Router};
use http::header::CONTENT_TYPE;

use crate::api::sharing::*;
use crate::api::RequestContext;
use crate::models::sharing::v1::*;
use crate::{Error, Recipient, Result};

/// Create a new [Router] for the Delta Sharing REST API.
pub fn get_router<T: SharingDiscoveryHandler + SharingQueryHandler + Clone>(state: T) -> Router {
    Router::new()
        .route("/shares", get(list_shares::<T>))
        .route("/shares/{share}", get(get_share::<T>))
        .route("/shares/{share}/schemas", get(list_sharing_schemas::<T>))
        .route("/shares/{share}/all-tables", get(list_share_tables::<T>))
        .route(
            "/shares/{share}/schemas/{name}/tables",
            get(list_schema_tables::<T>),
        )
        .route(
            "/shares/{share}/schemas/{schema}/tables/{name}/version",
            get(get_table_version_correct::<T>),
        )
        .route(
            "/shares/{share}/schemas/{schema}/tables/{name}/metadata",
            get(get_table_metadata_correct::<T>),
        )
        .with_state(state)
}

async fn get_table_version_correct<T: SharingQueryHandler>(
    State(handler): State<T>,
    Extension(recipient): Extension<Recipient>,
    request: GetTableVersionRequest,
) -> Result<Response> {
    let ctx = RequestContext { recipient };
    let result = handler.get_table_version(request, ctx).await?;
    Response::builder()
        .header("Delta-Table-Version", result.version)
        .body(Body::empty())
        .map_err(|e| Error::generic(e.to_string()))
}

async fn get_table_metadata_correct<T: SharingQueryHandler>(
    State(handler): State<T>,
    Extension(recipient): Extension<Recipient>,
    request: GetTableMetadataRequest,
) -> Result<Response> {
    let ctx = RequestContext { recipient };
    let result = handler.get_table_metadata(request, ctx).await?;
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
