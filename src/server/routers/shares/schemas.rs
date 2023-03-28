use crate::server::entities::schema::Name as SchemaName;
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
        (status = 200, description = "List matching share(s) successfully", body = SharesSchemasListResponse),
        (status = 401, description = "Authorization failed", body = ErrorMessage),
        (status = 422, description = "Validation failed", body = ErrorMessage),
        (status = 500, description = "Error occured while selecting share(s) on database", body = ErrorMessage),
    )
)]
pub async fn list(
    Extension(state): Extension<SharedState>,
    Path(SharesSchemasListParams { share }): Path<SharesSchemasListParams>,
    query: Query<SharesSchemasListQuery>,
) -> Result<Response, Error> {
    let share = ShareName::new(share).map_err(|_| Error::ValidationFailed)?;
    let limit = if let Some(limit) = &query.max_results {
        let limit = usize::try_from(*limit).map_err(|_| Error::ValidationFailed)?;
        limit
    } else {
        DEFAULT_PAGE_RESULTS
    };
    let after = if let Some(name) = &query.page_token {
        let after = SchemaName::new(name).map_err(|_| Error::ValidationFailed)?;
        Some(after)
    } else {
        None
    };
    let schemas = SchemaService::query(
        &share,
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
