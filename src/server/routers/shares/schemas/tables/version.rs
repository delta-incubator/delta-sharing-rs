use crate::server::entities::schema::Name as SchemaName;
use crate::server::entities::share::Name as ShareName;
use crate::server::entities::table::Name as TableName;
use crate::server::routers::SharedState;
use crate::server::services::error::Error;
use crate::server::services::table::Service as TableService;
use crate::server::utilities::deltalake::Utility as DeltalakeUtility;
use anyhow::anyhow;
use axum::extract::Extension;
use axum::extract::Path;
use axum::extract::Query;
use axum::http::header::HeaderMap;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use utoipa::IntoParams;

const HEADER_NAME: &str = "Delta-Table-Version";

#[derive(Debug, serde::Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct SharesSchemasTablesVersionGetParams {
    share: String,
    schema: String,
    table: String,
}

#[derive(Debug, serde::Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct SharesSchemasTablesVersionGetQuery {
    pub starting_timestamp: Option<String>,
}

#[utoipa::path(
    get,
    path = "/shares/{share}/schemas/{schema}/tables/{table}/version",
    tag = "official",
    operation_id = "GetTableVersion",
    params(SharesSchemasTablesVersionGetParams),
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
    Query(query): Query<SharesSchemasTablesVersionGetQuery>,
) -> Result<Response, Error> {
    let starting_timestamp = if let Some(starting_timestamp) = &query.starting_timestamp {
        let Ok(starting_timestamp) = DeltalakeUtility::datetime_yyyy_mm_dd_hh_mm_ss(starting_timestamp) else {
            tracing::error!("requested starting timestamp is malformed");
	        return Err(Error::ValidationFailed);
	    };
        Some(starting_timestamp)
    } else {
        None
    };
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
	    return Err(anyhow!("error occured while selecting table(s)").into());
    };
    let Some(table) = table else {
        tracing::error!("requested table does not exist");
	    return Err(Error::NotFound);
    };
    let Ok(mut table) = DeltalakeUtility::open_table(&table.location).await else {
        tracing::error!("request is not handled correctly due to a server error while loading delta table");
    	return Err(anyhow!("error occured while selecting table(s)").into());
    };
    if let Some(starting_timestamp) = starting_timestamp {
        let Ok(_) = table.load_with_datetime(starting_timestamp).await else {
            tracing::error!("request is not handled correctly due to a server error while time-traveling delta table");
    	    return Err(anyhow!("error occured while selecting table(s)").into());
    	};
    }
    let mut headers = HeaderMap::new();
    headers.insert(HEADER_NAME, table.version().into());
    tracing::info!("delta table version was successfully returned");
    Ok((StatusCode::OK, headers).into_response())
}
