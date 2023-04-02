use crate::config;
use crate::server::entities::schema::Name as SchemaName;
use crate::server::entities::share::Name as ShareName;
use crate::server::entities::table::Name as TableName;
use crate::server::routers::SharedState;
use crate::server::services::deltalake::Metadata;
use crate::server::services::deltalake::Protocol;
use crate::server::services::deltalake::Service as DeltalakeService;
use crate::server::services::error::Error;
use crate::server::services::table::Service as TableService;
use anyhow::anyhow;
use axum::extract::Extension;
use axum::extract::Path;
use axum::http::header;
use axum::http::header::HeaderValue;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::BoxError;
use axum_extra::json_lines::JsonLines;
use deltalake::delta::open_table_with_storage_options;
use futures_util::stream::Stream;
use serde_json::json;
use std::collections::hash_map::HashMap;
use utoipa::IntoParams;
use utoipa::ToSchema;

const HEADER_NAME: &str = "Delta-Table-Version";

#[derive(Debug, serde::Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct SharesSchemasTablesMetadataGetParams {
    share: String,
    schema: String,
    table: String,
}

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SharesSchemasTablesMetadataGetProtocolResponse {
    pub protocol: Protocol,
}

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SharesSchemasTablesMetadataGetMetadataResponse {
    pub meta_data: Metadata,
}

fn to_stream(
    protocol: serde_json::Value,
    metadata: serde_json::Value,
) -> impl Stream<Item = Result<serde_json::Value, BoxError>> {
    futures_util::stream::iter(vec![Ok(protocol), Ok(metadata)])
}

#[utoipa::path(
    get,
    path = "/shares/{share}/schemas/{schema}/tables/{table}/metadata",
    responses(
        (status = 200, description = "The table metadata was successfully returned.", body = JsonLines),
        (status = 400, description = "The request is malformed.", body = ErrorMessage),
        (status = 401, description = "The request is unauthenticated. The bearer token is missing or incorrect.", body = ErrorMessage),
        (status = 403, description = "The request is forbidden from being fulfilled.", body = ErrorMessage),
        (status = 404, description = "The requested resource does not exist.", body = ErrorMessage),
        (status = 500, description = "The request is not handled correctly due to a server error.", body = ErrorMessage),
    )
)]
#[tracing::instrument(skip(state))]
pub async fn get(
    Extension(state): Extension<SharedState>,
    Path(params): Path<SharesSchemasTablesMetadataGetParams>,
) -> Result<Response, Error> {
    let Ok(share) = ShareName::new(params.share) else {
        tracing::error!("requested share data is malformed");
	return Err(Error::ValidationFailed);
    };
    let Ok(schema) = SchemaName::new(params.schema) else {
        tracing::error!("requested schema data is malformed");
	return Err(Error::ValidationFailed);
    };
    let Ok(table) = TableName::new(params.table) else {
        tracing::error!("requested table data is malformed");
	return Err(Error::ValidationFailed);
    };
    let Ok(table) = TableService::query_by_fqn(
        &share,
        &schema,
        &table,
        &state.pg_pool,
    ).await else {
        tracing::error!("request is not handled correctly due to a server error while selecting table");
	return Err(anyhow!("error occured while selecting tables(s)").into());
    };
    let Some(table) = table else {
        tracing::error!("requested table does not exist");
	return Err(Error::NotFound);
    };
    let options = HashMap::from([(
        String::from("google_service_account_path"),
        config::fetch::<String>("gcp_sa_private_key"),
    )]);
    let Ok(table) = open_table_with_storage_options(&table.location, options).await else {
        tracing::error!("request is not handled correctly due to a server error while loading delta table");
	return Err(anyhow!("error occured while selecting tables(s)").into());
    };
    let Ok(metadata) = table.get_metadata() else {
        tracing::error!("request is not handled correctly due to a server error while loading delta table metadata");
    	return Err(anyhow!("error occured while selecting tables(s)").into());
    };
    let Ok(metadata) = DeltalakeService::metadata_from(metadata.to_owned()) else {
        tracing::error!("request is not handled correctly due to a server error while loading delta table metadata");
    	return Err(anyhow!("error occured while selecting tables(s)").into());
    };
    let Ok(protocol) = DeltalakeService::new_protocol() else {
        tracing::error!("request is not handled correctly due to a server error while loading delta table protocol");
    	return Err(anyhow!("error occured while selecting tables(s)").into());
    };
    let protocol = SharesSchemasTablesMetadataGetProtocolResponse { protocol: protocol };
    let metadata = SharesSchemasTablesMetadataGetMetadataResponse {
        meta_data: metadata,
    };
    let mut response = (
        StatusCode::OK,
        JsonLines::new(to_stream(json!(protocol), json!(metadata))),
    )
        .into_response();
    let headers = response.headers_mut();
    headers.insert(HEADER_NAME, table.version().into());
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/x-ndjson"),
    );
    tracing::info!("delta table metadata was successfully returned");
    Ok(response)
}
