use crate::error::Error;
use crate::server::entities::share::Entity as ShareEntity;
use crate::server::entities::share::Name as ShareName;
use crate::server::interactors::SharedState;
use crate::utils::jwt::Claims;
use axum::extract::Extension;
use axum::extract::Json;
use axum::extract::Path;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use tracing::error;
use tracing::info;
use utoipa::IntoParams;
use utoipa::ToSchema;

const DEFAULT_PAGE_RESULTS: usize = 10;

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Share {
    pub id: String,
    pub name: String,
}

#[derive(serde::Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct SharesGetParams {
    name: String,
}

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SharesGetResponse {
    pub share: Share,
}

#[utoipa::path(
    get,
    path = "/shares/{name}",
    params(
        SharesGetParams,
    ),
    responses(
        (status = 200, description = "Show matching share successfully", body = SharesGetResponse),
        (status = 401, description = "Authorization failed", body = ErrorResponse),
        (status = 404, description = "Share not found", body = ErrorResponse),
        (status = 422, description = "Validation failed", body = ErrorResponse),
        (status = 500, description = "Error occured while selecting share on database", body = ErrorResponse),
    )
)]
pub async fn get(
    _claims: Claims,
    Extension(state): Extension<SharedState>,
    Path(SharesGetParams { name }): Path<SharesGetParams>,
) -> Result<Response, Error> {
    let name = if let Ok(name) = ShareName::new(name) {
        name
    } else {
        error!("failed to validate share name");
        return Err(Error::ValidationFailed);
    };
    match ShareEntity::find_by_name(&name, &state.pg_pool).await? {
        Some(entity) => {
            info!(r#"found share name: "{}""#, entity.name().as_str());
            Ok((
                StatusCode::OK,
                Json(SharesGetResponse {
                    share: Share {
                        id: entity.id().to_string(),
                        name: entity.name().to_string(),
                    },
                }),
            )
                .into_response())
        }
        None => {
            error!(r#"failed to find share name: "{}""#, name.as_str());
            return Err(Error::NotFound);
        }
    }
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
        (status = 401, description = "Authorization failed", body = ErrorResponse),
        (status = 422, description = "Validation failed", body = ErrorResponse),
        (status = 500, description = "Error occured while selecting share(s) on database", body = ErrorResponse),
    )
)]
pub async fn list(
    _claims: Claims,
    Extension(state): Extension<SharedState>,
    query: Query<SharesListQuery>,
) -> Result<Response, Error> {
    let limit = if let Some(limit) = &query.max_results {
        if let Ok(limit) = usize::try_from(*limit) {
            limit
        } else {
            error!("failed to validate max results query");
            return Err(Error::ValidationFailed);
        }
    } else {
        DEFAULT_PAGE_RESULTS
    };
    let after = if let Some(name) = &query.page_token {
        if let Ok(name) = ShareName::new(name) {
            Some(name)
        } else {
            error!("failed to validate share name");
            return Err(Error::ValidationFailed);
        }
    } else {
        None
    };
    let entities = ShareEntity::list(&((limit + 1) as i64), &after, &state.pg_pool).await?;
    if entities.len() == limit + 1 {
        let next = &entities[limit];
        let entities = &entities[..limit];
        info!(r"found {} share(s)", entities.len());
        return Ok((
            StatusCode::OK,
            Json(SharesListResponse {
                items: entities
                    .iter()
                    .map(|entity| Share {
                        id: entity.id().to_string(),
                        name: entity.name().to_string(),
                    })
                    .collect(),
                next_page_token: next.name().to_string(),
            }),
        )
            .into_response());
    }
    info!(r"found {} share(s)", entities.len());
    Ok((
        StatusCode::OK,
        Json(SharesListResponse {
            items: entities
                .iter()
                .map(|entity| Share {
                    id: entity.id().to_string(),
                    name: entity.name().to_string(),
                })
                .collect(),
            next_page_token: None.unwrap_or_default(),
        }),
    )
        .into_response())
}
