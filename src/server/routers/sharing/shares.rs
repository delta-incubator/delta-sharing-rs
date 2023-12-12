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
use crate::server::routers::SharedState;
use crate::server::services::error::Error;
use crate::server::services::share::Service as ShareService;
use crate::server::services::share::Share;

pub mod all_tables;
pub mod schemas;

const DEFAULT_PAGE_RESULTS: usize = 10;

#[derive(Debug, serde::Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct SharingSharesGetParams {
    share: String,
}

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SharingSharesGetResponse {
    pub share: Share,
}

#[utoipa::path(
    get,
    path = "/sharing/shares/{share}",
    tag = "sharing",
    operation_id = "SharingGetShare",
    params(SharingSharesGetParams),
    responses(
        (status = 200, description = "The share's metadata was successfully returned.", body = SharingSharesGetResponse),
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
    Path(params): Path<SharingSharesGetParams>,
) -> Result<Response, Error> {
    let Ok(share) = ShareName::new(params.share) else {
        tracing::error!("requested share data is malformed");
        return Err(Error::ValidationFailed);
    };
    let Ok(share) = ShareService::query_by_name(&share, &state.pg_pool).await else {
        tracing::error!(
            "request is not handled correctly due to a server error while selecting share"
        );
        return Err(anyhow!("error occured while selecting share").into());
    };
    let Some(share) = share else {
        tracing::error!("requested share does not exist");
        return Err(Error::NotFound);
    };
    tracing::info!("share's metadata was successfully returned");
    Ok((StatusCode::OK, Json(SharingSharesGetResponse { share })).into_response())
}

#[derive(Debug, serde::Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct SharingSharesListQuery {
    pub max_results: Option<i64>,
    pub page_token: Option<String>,
}

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SharingSharesListResponse {
    pub items: Vec<Share>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[utoipa::path(
    get,
    path = "/sharing/shares",
    operation_id = "SharingListShares",
    tag = "sharing",
    params(SharingSharesListQuery),
    responses(
        (status = 200, description = "The shares were successfully returned.", body = SharingSharesListResponse),
        (status = 400, description = "The request is malformed.", body = ErrorMessage),
        (status = 401, description = "The request is unauthenticated. The bearer token is missing or incorrect.", body = ErrorMessage),
        (status = 403, description = "The request is forbidden from being fulfilled.", body = ErrorMessage),
        (status = 500, description = "The request is not handled correctly due to a server error.", body = ErrorMessage),
    )
)]
#[tracing::instrument(skip(state))]
pub async fn list(
    Extension(state): Extension<SharedState>,
    Query(query): Query<SharingSharesListQuery>,
) -> Result<Response, Error> {
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
        ShareName::new(name).ok()
    } else {
        None
    };
    let Ok(shares) =
        ShareService::query(Some(&((limit + 1) as i64)), after.as_ref(), &state.pg_pool).await
    else {
        tracing::error!(
            "request is not handled correctly due to a server error while selecting shares"
        );
        return Err(anyhow!("error occured while selecting share(s)").into());
    };
    if shares.len() == limit + 1 {
        let next = &shares[limit];
        let shares = &shares[..limit];
        tracing::info!("shares were successfully returned");
        return Ok((
            StatusCode::OK,
            Json(SharingSharesListResponse {
                items: shares.to_vec(),
                next_page_token: next.name.clone().into(),
            }),
        )
            .into_response());
    }
    tracing::info!("shares were successfully returned");
    Ok((
        StatusCode::OK,
        Json(SharingSharesListResponse {
            items: shares,
            next_page_token: None,
        }),
    )
        .into_response())
}
