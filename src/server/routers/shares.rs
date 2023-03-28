pub mod schemas;
use crate::server::entities::share::Name as ShareName;
use crate::server::routers::SharedState;
use crate::server::services::error::Error;
use crate::server::services::share::Service as ShareService;
use crate::server::services::share::Share;
use anyhow::Context;
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
    params(
        SharesGetParams,
    ),
    responses(
        (status = 200, description = "Show matching share successfully", body = SharesGetResponse),
        (status = 401, description = "Authorization failed", body = ErrorMessage),
        (status = 404, description = "Share not found", body = ErrorMessage),
        (status = 422, description = "Validation failed", body = ErrorMessage),
        (status = 500, description = "Error occured while selecting share on database", body = ErrorMessage),
    )
)]
pub async fn get(
    Extension(state): Extension<SharedState>,
    Path(SharesGetParams { share }): Path<SharesGetParams>,
) -> Result<Response, Error> {
    let share = ShareName::new(share).map_err(|_| Error::ValidationFailed)?;
    let share = ShareService::query_by_name(&share, &state.pg_pool)
        .await
        .context("error occured while selecting share")?;
    let Some(share) = share else {
	return Err(Error::NotFound);
    };
    debug!(r#"found share id: "{}" name: "{}""#, &share.id, &share.name);
    Ok((StatusCode::OK, Json(SharesGetResponse { share: share })).into_response())
}

#[derive(serde::Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct SharesListQuery {
    pub max_results: Option<i64>,
    pub page_token: Option<String>,
}

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SharesListResponse {
    pub items: Vec<Share>,
    pub next_page_token: String,
}

#[utoipa::path(
    get,
    path = "/shares",
    params(
        SharesListQuery,
    ),
    responses(
        (status = 200, description = "List matching share(s) successfully", body = SharesListResponse),
        (status = 401, description = "Authorization failed", body = ErrorMessage),
        (status = 422, description = "Validation failed", body = ErrorMessage),
        (status = 500, description = "Error occured while selecting share(s) on database", body = ErrorMessage),
    )
)]
pub async fn list(
    Extension(state): Extension<SharedState>,
    query: Query<SharesListQuery>,
) -> Result<Response, Error> {
    let limit = if let Some(limit) = &query.max_results {
        let limit = usize::try_from(*limit).map_err(|_| Error::ValidationFailed)?;
        limit
    } else {
        DEFAULT_PAGE_RESULTS
    };
    let after = if let Some(name) = &query.page_token {
        let after = ShareName::new(name).map_err(|_| Error::ValidationFailed)?;
        Some(after)
    } else {
        None
    };
    let shares = ShareService::query(Some(&((limit + 1) as i64)), after.as_ref(), &state.pg_pool)
        .await
        .context("error occured while selecting share(s)")?;
    if shares.len() == limit + 1 {
        let next = &shares[limit];
        let shares = &shares[..limit];
        debug!(r"found {} share(s)", shares.len());
        return Ok((
            StatusCode::OK,
            Json(SharesListResponse {
                items: shares.to_vec(),
                next_page_token: next.name.clone(),
            }),
        )
            .into_response());
    }
    debug!(r"found {} share(s)", shares.len());
    Ok((
        StatusCode::OK,
        Json(SharesListResponse {
            items: shares,
            next_page_token: None.unwrap_or_default(),
        }),
    )
        .into_response())
}
