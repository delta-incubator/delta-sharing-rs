use crate::server::entities::account::Entity as AccountEntity;
use crate::server::entities::account::Name as AccountName;
use crate::server::error::Error;
use crate::server::routers::SharedState;
use crate::server::schemas::account::Account;
use crate::server::utils::postgres::has_conflict;
use crate::server::utils::postgres::pg_error;
use anyhow::anyhow;
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

#[derive(serde::Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdminAccountsPostRequest {
    pub id: Option<String>,
    pub name: String,
    pub email: String,
    pub password: String,
    pub namespace: String,
    pub ttl: i64,
}

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdminAccountsPostResponse {
    pub account: Account,
}

#[utoipa::path(
    post,
    path = "/admin/accounts",
    request_body = AdminAccountsPostRequest,
    responses(
        (status = 201, description = "Registered account successfully", body = AdminAccountsPostResponse),
        (status = 401, description = "Authorization failed", body = Error),
        (status = 409, description = "Confliction occured", body = Error),
        (status = 422, description = "Validation failed", body = Error),
        (status = 500, description = "Error occured while creating account on database", body = Error),
    )
)]
pub async fn post(
    Extension(state): Extension<SharedState>,
    Json(payload): Json<AdminAccountsPostRequest>,
) -> Result<Response, Error> {
    let entity = AccountEntity::new(
        payload.id,
        payload.name,
        payload.email,
        payload.password,
        payload.namespace,
        payload.ttl,
    )
    .map_err(|_| Error::ValidationFailed)?;
    match pg_error(entity.register(&state.pg_pool).await)? {
        Ok(_) => {
            debug!(
                r#"updated account id: "{}" name: "{}""#,
                entity.id().as_uuid(),
                entity.name().as_str()
            );
            Ok((
                StatusCode::CREATED,
                Json(AdminAccountsPostResponse {
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
        Err(e) if has_conflict(&e) => Err(Error::Conflict),
        _ => Err(anyhow!("error occured while updating account").into()),
    }
}

#[derive(serde::Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct AdminAccountsGetParams {
    name: String,
}

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdminAccountsGetResponse {
    pub account: Account,
}

#[utoipa::path(
    get,
    path = "/admin/accounts/{name}",
    params(
        AdminAccountsGetParams,
    ),
    responses(
        (status = 200, description = "Show matching account successfully", body = AdminAccountsGetResponse),
        (status = 401, description = "Authorization failed", body = Error),
        (status = 404, description = "Account not found", body = Error),
        (status = 422, description = "Validation failed", body = Error),
        (status = 500, description = "Error occured while selecting account on database", body = Error),
    )
)]
pub async fn get(
    Extension(state): Extension<SharedState>,
    Path(AdminAccountsGetParams { name }): Path<AdminAccountsGetParams>,
) -> Result<Response, Error> {
    let name = AccountName::new(name).map_err(|_| Error::ValidationFailed)?;
    let entity = AccountEntity::find_by_name(&name, &state.pg_pool)
        .await
        .context("error occured while selecting account")?;
    let Some(entity) = entity else {
	return Err(Error::NotFound);
    };
    debug!(
        r#"found account id: "{}" name: "{}""#,
        entity.id().as_uuid(),
        entity.name().as_str()
    );
    Ok((
        StatusCode::OK,
        Json(AdminAccountsGetResponse {
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

#[derive(serde::Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct AdminAccountsListQuery {
    pub max_results: Option<i64>,
    pub page_token: Option<String>,
}

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdminAccountsListResponse {
    pub items: Vec<Account>,
    pub next_page_token: String,
}

#[utoipa::path(
    get,
    path = "/admin/accounts",
    params(
        AdminAccountsListQuery,
    ),
    responses(
        (status = 200, description = "List matching account(s) successfully", body = AdminAccountsListResponse),
        (status = 401, description = "Authorization failed", body = Error),
        (status = 422, description = "Validation failed", body = Error),
        (status = 500, description = "Error occured while selecting account(s) on database", body = Error),
    )
)]
pub async fn list(
    Extension(state): Extension<SharedState>,
    query: Query<AdminAccountsListQuery>,
) -> Result<Response, Error> {
    let limit = if let Some(limit) = &query.max_results {
        let limit = usize::try_from(*limit).map_err(|_| Error::ValidationFailed)?;
        limit
    } else {
        DEFAULT_PAGE_RESULTS
    };
    let after = if let Some(name) = &query.page_token {
        let after = AccountName::new(name).map_err(|_| Error::ValidationFailed)?;
        Some(after)
    } else {
        None
    };
    let entities = AccountEntity::list(&((limit + 1) as i64), &after, &state.pg_pool)
        .await
        .context("error occured while selecting account(s)")?;
    if entities.len() == limit + 1 {
        let next = &entities[limit];
        let entities = &entities[..limit];
        debug!(r"found {} account(s)", entities.len());
        return Ok((
            StatusCode::OK,
            Json(AdminAccountsListResponse {
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
    debug!(r"found {} account(s)", entities.len());
    Ok((
        StatusCode::OK,
        Json(AdminAccountsListResponse {
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
