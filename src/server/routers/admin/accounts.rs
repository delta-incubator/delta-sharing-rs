use crate::server::entities::account::Entity as AccountEntity;
use crate::server::entities::account::Name as AccountName;
use crate::server::routers::SharedState;
use crate::server::services::account::Account;
use crate::server::services::account::Service as AccountService;
use crate::server::services::error::Error;
use crate::server::utilities::postgres::Utility as PostgresUtility;
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
        (status = 401, description = "Authorization failed", body = ErrorMessage),
        (status = 409, description = "Confliction occured", body = ErrorMessage),
        (status = 422, description = "Validation failed", body = ErrorMessage),
        (status = 500, description = "Error occured while creating account on database", body = ErrorMessage),
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
    match PostgresUtility::error(entity.save(&state.pg_pool).await)? {
        Ok(_) => {
            debug!(
                r#"updated account id: "{}" name: "{}""#,
                entity.id().as_uuid(),
                entity.name().as_str()
            );
            Ok((
                StatusCode::CREATED,
                Json(AdminAccountsPostResponse {
                    account: Account::from(entity),
                }),
            )
                .into_response())
        }
        Err(e) if PostgresUtility::is_conflict(&e) => Err(Error::Conflict),
        _ => Err(anyhow!("error occured while updating account").into()),
    }
}

#[derive(serde::Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct AdminAccountsGetParams {
    account: String,
}

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdminAccountsGetResponse {
    pub account: Account,
}

#[utoipa::path(
    get,
    path = "/admin/accounts/{account}",
    params(
        AdminAccountsGetParams,
    ),
    responses(
        (status = 200, description = "Show matching account successfully", body = AdminAccountsGetResponse),
        (status = 401, description = "Authorization failed", body = ErrorMessage),
        (status = 404, description = "Account not found", body = ErrorMessage),
        (status = 422, description = "Validation failed", body = ErrorMessage),
        (status = 500, description = "Error occured while selecting account on database", body = ErrorMessage),
    )
)]
pub async fn get(
    Extension(state): Extension<SharedState>,
    Path(AdminAccountsGetParams { account }): Path<AdminAccountsGetParams>,
) -> Result<Response, Error> {
    let account = AccountName::new(account).map_err(|_| Error::ValidationFailed)?;
    let account = AccountService::query_by_name(&account, &state.pg_pool)
        .await
        .context("error occured while querying account")?;
    let Some(account) = account else {
	return Err(Error::NotFound);
    };
    debug!(r#"found account name: "{}""#, &account.name);
    Ok((StatusCode::OK, Json(AdminAccountsGetResponse { account })).into_response())
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
        (status = 401, description = "Authorization failed", body = ErrorMessage),
        (status = 422, description = "Validation failed", body = ErrorMessage),
        (status = 500, description = "Error occured while selecting account(s) on database", body = ErrorMessage),
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
    let accounts =
        AccountService::query(Some(&((limit + 1) as i64)), after.as_ref(), &state.pg_pool)
            .await
            .context("error occured while querying account(s)")?;
    if accounts.len() == limit + 1 {
        let next = &accounts[limit];
        let accounts = &accounts[..limit];
        debug!(r"found {} account(s)", accounts.len());
        return Ok((
            StatusCode::OK,
            Json(AdminAccountsListResponse {
                items: accounts.to_vec(),
                next_page_token: next.name.clone(),
            }),
        )
            .into_response());
    }
    debug!(r"found {} account(s)", accounts.len());
    Ok((
        StatusCode::OK,
        Json(AdminAccountsListResponse {
            items: accounts,
            next_page_token: None.unwrap_or_default(),
        }),
    )
        .into_response())
}
