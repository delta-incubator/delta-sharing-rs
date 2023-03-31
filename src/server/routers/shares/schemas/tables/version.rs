use crate::server::entities::schema::Name as SchemaName;
use crate::server::entities::share::Name as ShareName;
use crate::server::entities::table::Name as TableName;
use crate::server::routers::SharedState;
use crate::server::services::error::Error;
use crate::server::services::table::Service as TableService;
use anyhow::anyhow;
use axum::extract::Extension;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use deltalake::delta::open_table;
use utoipa::IntoParams;

use axum::http::header::HeaderMap;

const HEADER_NAME: &str = "Delta-Table-Version";

#[derive(Debug, serde::Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct SharesSchemasTablesVersionGetParams {
    share: String,
    schema: String,
    table: String,
}

#[utoipa::path(
    get,
    path = "/shares/{share}/schemas/{schema}/tables/{table}/version",
    responses(
        (status = 200, description = "The table version was successfully returned."),
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
    Path(params): Path<SharesSchemasTablesVersionGetParams>,
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
    let Ok(table) = open_table(&table.location).await else {
        tracing::error!("request is not handled correctly due to a server error while loading delta table");
	return Err(anyhow!("error occured while selecting tables(s)").into());
    };
    let mut headers = HeaderMap::new();
    headers.insert(HEADER_NAME, table.version().into());
    Ok((StatusCode::OK, headers).into_response())
}
