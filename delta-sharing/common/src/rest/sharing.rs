use axum::body::Body;
use axum::extract::{Extension, FromRequestParts, Path, Query, State};
use axum::http::request::Parts;
use axum::routing::{delete, get, post};
use axum::{response::Response, Router};
use axum::{RequestExt, RequestPartsExt};
use delta_sharing_derive::rest_handlers;
use http::header::CONTENT_TYPE;
use serde::Deserialize;

use crate::api::{
    RequestContext, SharingDiscoveryHandler, SharingExtensionHandler, SharingQueryHandler,
};
use crate::models::sharing::v1::*;
use crate::{Error, Recipient, Result};

/// Create a new [Router] for the Delta Sharing REST API.
pub fn get_router<
    T: SharingDiscoveryHandler + SharingQueryHandler + SharingExtensionHandler + Clone,
>(
    state: T,
) -> Router {
    Router::new()
        .route("/shares", post(create_share::<T>))
        .route("/shares", get(list_shares::<T>))
        .route("/shares/{share}", get(get_share::<T>))
        .route("/shares/{share}", delete(delete_share::<T>))
        .route("/shares/{share}/schemas", post(create_sharing_schema::<T>))
        .route("/shares/{share}/schemas", get(list_sharing_schemas::<T>))
        .route(
            "/shares/{share}/schemas/{name}",
            delete(delete_sharing_schema::<T>),
        )
        .route("/shares/{share}/all-tables", get(list_share_tables::<T>))
        .route(
            "/shares/{share}/schemas/{name}/tables",
            get(list_schema_tables::<T>),
        )
        .route(
            "/shares/{share}/schemas/{schema}/tables/{name}/version",
            get(get_table_version::<T>),
        )
        .route(
            "/shares/{share}/schemas/{schema}/tables/{name}/metadata",
            get(get_table_metadata::<T>),
        )
        .with_state(state)
}

impl<S: Send + Sync> FromRequestParts<S> for GetTableVersionRequest {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        #[derive(Deserialize)]
        struct GetTableVersionQuery {
            starting_timestamp: Option<String>,
        }
        let Query(query) = parts.extract::<Query<GetTableVersionQuery>>().await?;
        let Path((share, schema, name)) = parts.extract::<Path<(String, String, String)>>().await?;
        Ok(GetTableVersionRequest {
            share,
            schema,
            name,
            starting_timestamp: query.starting_timestamp,
        })
    }
}

async fn get_table_version<T: SharingQueryHandler>(
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

impl<S: Send + Sync> FromRequestParts<S> for GetTableMetadataRequest {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let Path((share, schema, name)) = parts.extract::<Path<(String, String, String)>>().await?;
        Ok(GetTableMetadataRequest {
            share,
            schema,
            name,
        })
    }
}

async fn get_table_metadata<T: SharingQueryHandler>(
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

rest_handlers!(
    SharingDiscoveryHandler, [
        ListSharesRequest, ListSharesResponse;
        GetShareRequest, Share with [
            name: path as String,
        ];
        ListSharingSchemasRequest, ListSharingSchemasResponse with [
            share: path as String,
        ];
        ListShareTablesRequest, ListShareTablesResponse with [
            name: path as String,
        ];
        ListSchemaTablesRequest, ListSchemaTablesResponse with [
            share: path as String,
            name: path as String,
        ];
    ]
);

rest_handlers!(
    SharingExtensionHandler, [
        CreateShareRequest, ShareInfo;
        DeleteShareRequest with [
            name: path as String,
            force: query as Option<bool>
        ];
        CreateSharingSchemaRequest, SharingSchemaInfo with [
            share: path as String,
        ];
        DeleteSharingSchemaRequest with [
            share: path as String,
            name: path as String,
        ];
    ]
);
