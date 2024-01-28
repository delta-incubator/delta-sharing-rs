use anyhow::anyhow;
use axum::extract::{Extension, Json, Path, Query};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use utoipa::{IntoParams, ToSchema};

use crate::server::entities::schema::Name as SchemaName;
use crate::server::entities::share::Entity as ShareEntity;
use crate::server::entities::share::Name as ShareName;
use crate::server::routers::SharedState;
use crate::server::services::error::Error;
use crate::server::services::schema::SchemaDetail;
use crate::server::services::schema::Service as SchemaService;

pub mod tables;

const DEFAULT_PAGE_RESULTS: usize = 10;

#[derive(Debug, serde::Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct SharesSchemasListParams {
    share: String,
}

#[derive(Debug, serde::Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct SharesSchemasListQuery {
    pub max_results: Option<i64>,
    pub page_token: Option<String>,
}

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SharesSchemasListResponse {
    pub items: Vec<SchemaDetail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[utoipa::path(
    get,
    path = "/shares/{share}/schemas",
    operation_id = "ListSchemas",
    tag = "official",
    params(
        SharesSchemasListParams,
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
#[tracing::instrument(skip(state))]
pub async fn list(
    Extension(state): Extension<SharedState>,
    Path(params): Path<SharesSchemasListParams>,
    Query(query): Query<SharesSchemasListQuery>,
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
        SchemaName::try_new(name).ok()
    } else {
        None
    };
    let Ok(schemas) = SchemaService::query_by_share_name(
        share.name(),
        Some(&((limit + 1) as i64)),
        after.as_ref(),
        &state.pg_pool,
    )
    .await
    else {
        tracing::error!(
            "request is not handled correctly due to a server error while selecting schemas"
        );
        return Err(anyhow!("error occured while selecting schema(s)").into());
    };
    if schemas.len() == limit + 1 {
        let next = &schemas[limit];
        let schemas = &schemas[..limit];
        tracing::info!("schemas were successfully returned");
        return Ok((
            StatusCode::OK,
            Json(SharesSchemasListResponse {
                items: schemas.to_vec(),
                next_page_token: next.name.clone().into(),
            }),
        )
            .into_response());
    }
    tracing::info!("schemas were successfully returned");
    Ok((
        StatusCode::OK,
        Json(SharesSchemasListResponse {
            items: schemas,
            next_page_token: None,
        }),
    )
        .into_response())
}
