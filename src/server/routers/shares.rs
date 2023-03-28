pub mod schemas;
use crate::server::entities::share::Name as ShareName;
use crate::server::routers::SharedState;
use crate::server::services::error::Error;
use crate::server::services::share::Service as ShareService;
use crate::server::services::share::Share;
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
        (status = 200, description = "The share's metadata was successfully returned.", body = SharesGetResponse),
        (status = 400, description = "The request is malformed.", body = ErrorMessage),
        (status = 401, description = "The request is unauthenticated. The bearer token is missing or incorrect.", body = ErrorMessage),
        (status = 403, description = "The request is forbidden from being fulfilled.", body = ErrorMessage),
        (status = 404, description = "The requested resource does not exist.", body = ErrorMessage),
        (status = 500, description = "The request is not handled correctly due to a server error.", body = ErrorMessage),
    )
)]
pub async fn get(
    Extension(state): Extension<SharedState>,
    Path(SharesGetParams { share }): Path<SharesGetParams>,
) -> Result<Response, Error> {
    let Ok(share) = ShareName::new(share) else {
	return Err(Error::ValidationFailed);
    };
    let Ok(share) = ShareService::query_by_name(&share, &state.pg_pool).await else {
        return Err(anyhow!("error occured while selecting share").into());
    };
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
        (status = 200, description = "The shares were successfully returned.", body = SharesListResponse),
        (status = 400, description = "The request is malformed.", body = ErrorMessage),
        (status = 401, description = "The request is unauthenticated. The bearer token is missing or incorrect.", body = ErrorMessage),
        (status = 403, description = "The request is forbidden from being fulfilled.", body = ErrorMessage),
        (status = 500, description = "The request is not handled correctly due to a server error.", body = ErrorMessage),
    )
)]
pub async fn list(
    Extension(state): Extension<SharedState>,
    Query(SharesListQuery {
        max_results,
        page_token,
    }): Query<SharesListQuery>,
) -> Result<Response, Error> {
    let limit = if let Some(limit) = &max_results {
        let Ok(limit) = usize::try_from(*limit) else {
	    return Err(Error::ValidationFailed);
	};
        limit
    } else {
        DEFAULT_PAGE_RESULTS
    };
    let after = if let Some(name) = &page_token {
        let Ok(after) = ShareName::new(name) else {
	    return Err(Error::ValidationFailed);
	};
        Some(after)
    } else {
        None
    };
    let Ok(shares) = ShareService::query(Some(&((limit + 1) as i64)), after.as_ref(), &state.pg_pool).await else {
        return Err(anyhow!("error occured while selecting share(s)").into());
    };
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
