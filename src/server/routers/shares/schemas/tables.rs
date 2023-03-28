use crate::server::entities::schema::Name as SchemaName;
use crate::server::entities::share::Entity as ShareEntity;
use crate::server::entities::share::Name as ShareName;
use crate::server::entities::table::Name as TableName;
use crate::server::routers::SharedState;
use crate::server::services::error::Error;
use crate::server::services::table::Service as TableService;
use crate::server::services::table::TableDetail;
use anyhow::anyhow;
use axum::extract::Extension;
use axum::extract::Json;
use axum::extract::Path;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use tracing::debug;
use utoipa::IntoParams;
use utoipa::ToSchema;

const DEFAULT_PAGE_RESULTS: usize = 10;

#[derive(serde::Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct SharesSchemasTablesListParams {
    share: String,
    schema: String,
}

#[derive(serde::Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct SharesSchemasTablesListQuery {
    pub max_results: Option<i64>,
    pub page_token: Option<String>,
}

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SharesSchemasTablesListResponse {
    pub items: Vec<TableDetail>,
    pub next_page_token: String,
}

#[utoipa::path(
    get,
    path = "/shares/{share}/schemas/{schema}/tables",
    params(
        SharesSchemasTablesListQuery,
    ),
    responses(
        (status = 200, description = "The tables were successfully returned.", body = SharesSchemasTablesListResponse),
        (status = 400, description = "The request is malformed.", body = ErrorMessage),
        (status = 401, description = "The request is unauthenticated. The bearer token is missing or incorrect.", body = ErrorMessage),
        (status = 403, description = "The request is forbidden from being fulfilled.", body = ErrorMessage),
        (status = 404, description = "The requested resource does not exist.", body = ErrorMessage),
        (status = 500, description = "The request is not handled correctly due to a server error.", body = ErrorMessage),
    )
)]
pub async fn list(
    Extension(state): Extension<SharedState>,
    Path(SharesSchemasTablesListParams { share, schema }): Path<SharesSchemasTablesListParams>,
    Query(SharesSchemasTablesListQuery {
        max_results,
        page_token,
    }): Query<SharesSchemasTablesListQuery>,
) -> Result<Response, Error> {
    let Ok(share) = ShareName::new(share) else {
	return Err(Error::ValidationFailed);
    };
    let Ok(share) = ShareEntity::load(&share, &state.pg_pool).await else {
        return Err(anyhow!("error occured while selecting share").into());
    };
    let Some(share) = share else {
	return Err(Error::NotFound);
    };
    let Ok(schema) = SchemaName::new(schema) else {
	return Err(Error::ValidationFailed);
    };
    let limit = if let Some(limit) = &max_results {
        let Ok(limit) = usize::try_from(*limit) else {
	    return Err(Error::ValidationFailed);
	};
        limit
    } else {
        DEFAULT_PAGE_RESULTS
    };
    let after = if let Some(name) = &page_token {
        let Ok(after) = TableName::new(name) else {
	    return Err(Error::ValidationFailed);
	};
        Some(after)
    } else {
        None
    };
    let Ok(tables) = TableService::query_by_share_and_schema_name(
        share.name(),
        &schema,
        Some(&((limit + 1) as i64)),
        after.as_ref(),
        &state.pg_pool,
    ).await else {
	return Err(anyhow!("error occured while selecting tables(s)").into());
    };
    if tables.len() == limit + 1 {
        let next = &tables[limit];
        let tables = &tables[..limit];
        debug!(r"found {} tables(s)", tables.len());
        return Ok((
            StatusCode::OK,
            Json(SharesSchemasTablesListResponse {
                items: tables.to_vec(),
                next_page_token: next.name.clone(),
            }),
        )
            .into_response());
    }
    debug!(r"found {} tables(s)", tables.len());
    Ok((
        StatusCode::OK,
        Json(SharesSchemasTablesListResponse {
            items: tables,
            next_page_token: None.unwrap_or_default(),
        }),
    )
        .into_response())
}
