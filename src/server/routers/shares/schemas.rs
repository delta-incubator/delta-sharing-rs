use crate::server::entities::schema::Name as SchemaName;
use crate::server::entities::share::Entity as ShareEntity;
use crate::server::entities::share::Name as ShareName;
use crate::server::routers::SharedState;
use crate::server::services::error::Error;
use crate::server::services::schema::SchemaDetail;
use crate::server::services::schema::Service as SchemaService;
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
pub struct SharesSchemasListParams {
    share: String,
}

#[derive(serde::Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct SharesSchemasListQuery {
    pub max_results: Option<i64>,
    pub page_token: Option<String>,
}

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SharesSchemasListResponse {
    pub items: Vec<SchemaDetail>,
    pub next_page_token: String,
}

#[utoipa::path(
    get,
    path = "/shares/{share}/schemas",
    params(
        SharesSchemasListQuery,
    ),
    responses(
        (status = 200, description = "The schemas were successfully returned.", body = SharesSchemasListResponse),
        (status = 400, description = "The request is malformed.", body = ErrorMessage),
        (status = 401, description = "The request is unauthenticated. The bearer token is missing or incorrect.", body = ErrorMessage),
        (status = 403, description = "The request is forbidden from being fulfilled.", body = ErrorMessage),
        (status = 404, description = "The requested resource does not exist.", body = ErrorMessage),
        (status = 500, description = "The request is not handled correctly due to a server error.", body = ErrorMessage),
    )
)]
pub async fn list(
    Extension(state): Extension<SharedState>,
    Path(SharesSchemasListParams { share }): Path<SharesSchemasListParams>,
    Query(SharesSchemasListQuery {
        max_results,
        page_token,
    }): Query<SharesSchemasListQuery>,
) -> Result<Response, Error> {
    let share = ShareName::new(share).map_err(|_| Error::ValidationFailed)?;
    let share = ShareEntity::load(&share, &state.pg_pool)
        .await
        .context("error occured while selecting share")?;
    let Some(share) = share else {
	return Err(Error::NotFound);
    };
    let limit = if let Some(limit) = &max_results {
        let limit = usize::try_from(*limit).map_err(|_| Error::ValidationFailed)?;
        limit
    } else {
        DEFAULT_PAGE_RESULTS
    };
    let after = if let Some(name) = &page_token {
        let after = SchemaName::new(name).map_err(|_| Error::ValidationFailed)?;
        Some(after)
    } else {
        None
    };
    let schemas = SchemaService::query(
        share.name(),
        Some(&((limit + 1) as i64)),
        after.as_ref(),
        &state.pg_pool,
    )
    .await
    .context("error occured while selecting schema(s)")?;
    if schemas.len() == limit + 1 {
        let next = &schemas[limit];
        let schemas = &schemas[..limit];
        debug!(r"found {} schema(s)", schemas.len());
        return Ok((
            StatusCode::OK,
            Json(SharesSchemasListResponse {
                items: schemas.to_vec(),
                next_page_token: next.name.clone(),
            }),
        )
            .into_response());
    }
    debug!(r"found {} schema(s)", schemas.len());
    Ok((
        StatusCode::OK,
        Json(SharesSchemasListResponse {
            items: schemas,
            next_page_token: None.unwrap_or_default(),
        }),
    )
        .into_response())
}
