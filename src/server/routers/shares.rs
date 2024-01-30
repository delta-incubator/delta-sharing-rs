use anyhow::anyhow;
use axum::extract::Extension;
use axum::extract::Json;
use axum::extract::Path;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use utoipa::IntoParams;
use utoipa::ToSchema;

use crate::server::entities::share::Name as ShareName;
use crate::server::routers::Pagination;
use crate::server::routers::SharedState;
use crate::server::services::error::Error;
use crate::server::services::share::Share;

pub mod all_tables;
pub mod schemas;

const DEFAULT_PAGE_RESULTS: usize = 10;

#[derive(Debug, serde::Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct SharesGetParams {
    share: String,
}

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SharesGetResponse {
    pub share: Share,
}

#[utoipa::path(
    get,
    path = "/shares/{share}",
    tag = "official",
    operation_id = "GetShare",
    params(SharesGetParams),
    responses(
        (status = 200, description = "The share's metadata was successfully returned.", body = SharesGetResponse),
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
    Path(params): Path<SharesGetParams>,
) -> Result<Response, Error> {
    let Ok(share_name) = ShareName::new(params.share) else {
        tracing::error!("requested share data is malformed");
        return Err(Error::ValidationFailed);
    };

    let Some(share) = state.state_store.get_share(&share_name.as_str()).await? else {
        tracing::error!("requested share does not exist");
        return Err(Error::NotFound);
    };

    tracing::info!("share's metadata was successfully returned");
    Ok((StatusCode::OK, Json(SharesGetResponse { share })).into_response())
}

#[derive(Debug, serde::Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct SharesListQuery {
    pub max_results: Option<u32>,
    pub page_token: Option<String>,
}

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SharesListResponse {
    pub items: Vec<Share>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[utoipa::path(
    get,
    path = "/shares",
    operation_id = "ListShares",
    tag = "official",
    params(SharesListQuery),
    responses(
        (status = 200, description = "The shares were successfully returned.", body = SharesListResponse),
        (status = 400, description = "The request is malformed.", body = ErrorMessage),
        (status = 401, description = "The request is unauthenticated. The bearer token is missing or incorrect.", body = ErrorMessage),
        (status = 403, description = "The request is forbidden from being fulfilled.", body = ErrorMessage),
        (status = 500, description = "The request is not handled correctly due to a server error.", body = ErrorMessage),
    )
)]
#[tracing::instrument(skip(state))]
pub async fn list(
    Extension(state): Extension<SharedState>,
    Query(query): Query<SharesListQuery>,
) -> Result<Response, Error> {
    let pagination = Pagination::new(query.max_results, query.page_token);
    let shares = state.state_store.list_shares(&pagination).await?;

    let res = SharesListResponse {
        items: shares.items().to_vec(),
        next_page_token: shares.next_page_token().map(ToOwned::to_owned),
    };
    tracing::info!("shares were successfully returned");
    Ok((StatusCode::OK, Json(res)).into_response())
}
