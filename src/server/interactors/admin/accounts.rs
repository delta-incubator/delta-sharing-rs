use crate::error::Error;
use crate::server::entities::account::Entity as AccountEntity;
use crate::server::entities::account::Name as AccountName;
use crate::server::interactors::SharedState;
use crate::utils::jwt::Claims;
use crate::utils::postgres::has_conflict;
use crate::utils::postgres::pg_error;
use anyhow::anyhow;
use axum::extract::Extension;
use axum::extract::Json;
use axum::extract::Path;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use tracing::error;
use tracing::info;
use tracing::warn;
use utoipa::IntoParams;
use utoipa::ToSchema;

const DEFAULT_PAGE_RESULTS: usize = 10;

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub name: String,
    pub email: String,
    pub namespace: String,
    pub ttl: i64,
}

#[derive(serde::Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PostRequest {
    pub id: Option<String>,
    pub name: String,
    pub email: String,
    pub password: String,
    pub namespace: String,
    pub ttl: i64,
}

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PostResponse {
    pub account: Account,
}

#[utoipa::path(
    post,
    path = "/admin/accounts",
    request_body = PostRequest,
    responses(
        (status = 201, description = "Registered account successfully", body = PostResponse),
        (status = 401, description = "Authorization failed", body = ErrorResponse),
        (status = 409, description = "Confliction occured", body = ErrorResponse),
        (status = 422, description = "Validation failed", body = ErrorResponse),
        (status = 500, description = "Error occured while creating account on database", body = ErrorResponse),
    )
)]
pub async fn post(
    _claims: Claims,
    Extension(state): Extension<SharedState>,
    Json(payload): Json<PostRequest>,
) -> Result<Response, Error> {
    let entity = if let Ok(entity) = AccountEntity::new(
        payload.id,
        payload.name,
        payload.email,
        payload.password,
        payload.namespace,
        payload.ttl,
    ) {
        entity
    } else {
        error!("failed to validate new account");
        return Err(Error::ValidationFailed);
    };
    match pg_error(entity.register(&state.pg_pool).await)? {
        Ok(_) => {
            info!(
                r#"updated account id: "{}" name: "{}""#,
                entity.id().as_uuid(),
                entity.name().as_str()
            );
            Ok((
                StatusCode::CREATED,
                Json(PostResponse {
                    account: Account {
                        name: entity.name().to_string(),
                        email: entity.email().to_string(),
                        namespace: entity.namespace().to_string(),
                        ttl: entity.ttl().to_i64(),
                    },
                }),
            )
                .into_response())
        }
        Err(e) if has_conflict(&e) => {
            warn!("failed to update account: {}", e);
            Err(Error::Conflict)
        }
        _ => Err(anyhow!("Unknown error").into()),
    }
}

#[derive(serde::Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct GetParams {
    name: String,
}

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetResponse {
    pub account: Account,
}

#[utoipa::path(
    get,
    path = "/admin/accounts/{name}",
    params(
        GetParams,
    ),
    responses(
        (status = 200, description = "Show matching account successfully", body = GetResponse),
        (status = 401, description = "Authorization failed", body = ErrorResponse),
        (status = 404, description = "Account not found", body = ErrorResponse),
        (status = 422, description = "Validation failed", body = ErrorResponse),
        (status = 500, description = "Error occured while selecting account on database", body = ErrorResponse),
    )
)]
pub async fn get(
    _claims: Claims,
    Extension(state): Extension<SharedState>,
    Path(GetParams { name }): Path<GetParams>,
) -> Result<Response, Error> {
    let name = if let Ok(name) = AccountName::new(name) {
        name
    } else {
        error!("failed to validate account name");
        return Err(Error::ValidationFailed);
    };
    match AccountEntity::find_by_name(&name, &state.pg_pool).await? {
        Some(entity) => {
            info!(r#"found account name: "{}""#, entity.name().as_str());
            Ok((
                StatusCode::OK,
                Json(GetResponse {
                    account: Account {
                        name: entity.name().to_string(),
                        email: entity.email().to_string(),
                        namespace: entity.namespace().to_string(),
                        ttl: entity.ttl().to_i64(),
                    },
                }),
            )
                .into_response())
        }
        None => {
            error!(r#"failed to find account name: "{}""#, name.as_str());
            return Err(Error::NotFound);
        }
    }
}

#[derive(serde::Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct ListQuery {
    pub max_results: Option<i64>,
    pub page_token: Option<String>,
}

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ListResponse {
    pub items: Vec<Account>,
    pub next_page_token: String,
}

#[utoipa::path(
    get,
    path = "/admin/accounts",
    params(
        ListQuery,
    ),
    responses(
        (status = 200, description = "List matching account(s) successfully", body = GetResponse),
        (status = 401, description = "Authorization failed", body = ErrorResponse),
        (status = 422, description = "Validation failed", body = ErrorResponse),
        (status = 500, description = "Error occured while selecting account(s) on database", body = ErrorResponse),
    )
)]
pub async fn list(
    _claims: Claims,
    Extension(state): Extension<SharedState>,
    query: Query<ListQuery>,
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
        if let Ok(name) = AccountName::new(name) {
            Some(name)
        } else {
            error!("failed to validate account name");
            return Err(Error::ValidationFailed);
        }
    } else {
        None
    };
    let entities = AccountEntity::list(&((limit + 1) as i64), &after, &state.pg_pool).await?;
    if entities.len() == limit + 1 {
        let next = &entities[limit];
        let entities = &entities[..limit];
        info!(r"found {} account(s)", entities.len());
        return Ok((
            StatusCode::OK,
            Json(ListResponse {
                items: entities
                    .iter()
                    .map(|entity| Account {
                        name: entity.name().to_string(),
                        email: entity.email().to_string(),
                        namespace: entity.namespace().to_string(),
                        ttl: entity.ttl().to_i64(),
                    })
                    .collect(),
                next_page_token: next.name().to_string(),
            }),
        )
            .into_response());
    }
    info!(r"found {} account(s)", entities.len());
    Ok((
        StatusCode::OK,
        Json(ListResponse {
            items: entities
                .iter()
                .map(|entity| Account {
                    name: entity.name().to_string(),
                    email: entity.email().to_string(),
                    namespace: entity.namespace().to_string(),
                    ttl: entity.ttl().to_i64(),
                })
                .collect(),
            next_page_token: None.unwrap_or_default(),
        }),
    )
        .into_response())
}
