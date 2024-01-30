use axum::extract::{Extension, Json, Path, Query};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use utoipa::{IntoParams, ToSchema};

use crate::server::catalog::Pagination;
use crate::server::entities::share::Name as ShareName;
use crate::server::routers::SharedState;
use crate::server::services::error::Error;
use crate::server::services::table::TableDetail;

const DEFAULT_PAGE_RESULTS: usize = 10;

#[derive(Debug, serde::Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct SharesAllTablesListParams {
    share: String,
}

#[derive(Debug, serde::Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct SharesAllTablesListQuery {
    pub max_results: Option<u32>,
    pub page_token: Option<String>,
}

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SharesAllTablesListResponse {
    pub items: Vec<TableDetail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[utoipa::path(
    get,
    path = "/shares/{share}/all-tables",
    operation_id = "ListALLTables",
    tag = "official",
    params(
        SharesAllTablesListParams,
        SharesAllTablesListQuery,
    ),
    responses(
        (status = 200, description = "The tables were successfully returned.", body = SharesAllTablesListResponse),
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
    Path(params): Path<SharesAllTablesListParams>,
    Query(query): Query<SharesAllTablesListQuery>,
) -> Result<Response, Error> {
    let Ok(share) = ShareName::try_new(params.share) else {
        tracing::error!("requested share data is malformed");
        return Err(Error::ValidationFailed);
    };

    let pagination = Pagination::new(query.max_results, query.page_token);
    let tables = state
        .share_store
        .list_tables_in_share(share.as_str(), &pagination)
        .await?;

    let res = SharesAllTablesListResponse {
        items: tables
            .items()
            .iter()
            .map(|t| TableDetail {
                name: t.name.to_string(),
                schema: String::new(),
                share: String::new(),
            })
            .collect::<Vec<_>>(),
        next_page_token: tables.next_page_token().map(|s| s.to_string()),
    };
    tracing::info!("tables were successfully returned");
    Ok((StatusCode::OK, Json(res)).into_response())
}
