use anyhow::anyhow;
use axum::extract::{Extension, Json, Path, Query};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use utoipa::{IntoParams, ToSchema};

use crate::server::entities::schema::Name as SchemaName;
use crate::server::entities::share::Entity as ShareEntity;
use crate::server::entities::share::Name as ShareName;
use crate::server::entities::table::Name as TableName;
use crate::server::routers::SharedState;
use crate::server::services::error::Error;
use crate::server::services::table::Service as TableService;
use crate::server::services::table::TableDetail;

pub mod metadata;
pub mod query;
pub mod version;

const DEFAULT_PAGE_RESULTS: usize = 10;

#[derive(Debug, serde::Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct SharesSchemasTablesListParams {
    share: String,
    schema: String,
}

#[derive(Debug, serde::Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct SharesSchemasTablesListQuery {
    pub max_results: Option<i64>,
    pub page_token: Option<String>,
}

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SharesSchemasTablesListResponse {
    pub items: Vec<TableDetail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[utoipa::path(
    get,
    path = "/shares/{share}/schemas/{schema}/tables",
    operation_id = "ListTables",
    tag = "official",
    params(
        SharesSchemasTablesListParams,
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
#[tracing::instrument(skip(state))]
pub async fn list(
    Extension(state): Extension<SharedState>,
    Path(params): Path<SharesSchemasTablesListParams>,
    Query(query): Query<SharesSchemasTablesListQuery>,
) -> Result<Response, Error> {
    let Ok(share) = ShareName::try_new(params.share) else {
        tracing::error!("requested share data is malformed");
        return Err(Error::ValidationFailed);
    };
    let Ok(share) = ShareEntity::load(&share, &state.pg_pool).await else {
        tracing::error!(
            "request is not handled correctly due to a server error while selecting share"
        );
        return Err(anyhow!("error occured while selecting share").into());
    };
    let Some(share) = share else {
        tracing::error!("requested share does not exist");
        return Err(Error::NotFound);
    };
    let Ok(schema) = SchemaName::try_new(params.schema) else {
        tracing::error!("requested schema data is malformed");
        return Err(Error::ValidationFailed);
    };
    let limit = if let Some(limit) = &query.max_results {
        let Ok(limit) = usize::try_from(*limit) else {
            tracing::error!("requested limit is malformed");
            return Err(Error::ValidationFailed);
        };
        limit
    } else {
        DEFAULT_PAGE_RESULTS
    };
    let after = if let Some(name) = &query.page_token {
        TableName::try_new(name).ok()
    } else {
        None
    };
    let Ok(tables) = TableService::query_by_share_and_schema_name(
        share.name(),
        &schema,
        Some(&((limit + 1) as i64)),
        after.as_ref(),
        &state.pg_pool,
    )
    .await
    else {
        tracing::error!(
            "request is not handled correctly due to a server error while selecting tables"
        );
        return Err(anyhow!("error occured while selecting tables(s)").into());
    };
    if tables.len() == limit + 1 {
        let next = &tables[limit];
        let tables = &tables[..limit];
        tracing::info!("tables were successfully returned");
        return Ok((
            StatusCode::OK,
            Json(SharesSchemasTablesListResponse {
                items: tables.to_vec(),
                next_page_token: next.name.clone().into(),
            }),
        )
            .into_response());
    }
    tracing::info!("tables were successfully returned");
    Ok((
        StatusCode::OK,
        Json(SharesSchemasTablesListResponse {
            items: tables,
            next_page_token: None,
        }),
    )
        .into_response())
}
