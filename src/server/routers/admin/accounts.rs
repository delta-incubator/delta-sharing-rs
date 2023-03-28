use crate::server::entities::account::Entity as AccountEntity;
use crate::server::entities::account::Name as AccountName;
use crate::server::routers::SharedState;
use crate::server::services::account::Account;
use crate::server::services::account::Service as AccountService;
use crate::server::services::error::Error;
use crate::server::utilities::postgres::Utility as PostgresUtility;
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
        (status = 201, description = "The account was successfully registered.", body = AdminAccountsPostResponse),
        (status = 400, description = "The request is malformed.", body = ErrorMessage),
        (status = 401, description = "The request is unauthenticated. The bearer token is missing or incorrect.", body = ErrorMessage),
        (status = 409, description = "The account was already registered.", body = ErrorMessage),
        (status = 500, description = "The request is not handled correctly due to a server error.", body = ErrorMessage),
    )
)]
pub async fn post(
    Extension(state): Extension<SharedState>,
    Json(AdminAccountsPostRequest {
        id,
        name,
        email,
        password,
        namespace,
        ttl,
    }): Json<AdminAccountsPostRequest>,
) -> Result<Response, Error> {
    let Ok(account) = AccountEntity::new(id, name, email, password, namespace, ttl) else {
        return Err(Error::ValidationFailed);
    };
    match PostgresUtility::error(account.save(&state.pg_pool).await)? {
        Ok(_) => {
            debug!(
                r#"updated account id: "{}" name: "{}""#,
                account.id().as_uuid(),
                account.name().as_str()
            );
            Ok((
                StatusCode::CREATED,
                Json(AdminAccountsPostResponse {
                    account: Account::from(account),
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
        (status = 200, description = "The account's metadata was successfully returned.", body = AdminAccountsGetResponse),
        (status = 400, description = "The request is malformed.", body = ErrorMessage),
        (status = 401, description = "The request is unauthenticated. The bearer token is missing or incorrect.", body = ErrorMessage),
        (status = 403, description = "The request is forbidden from being fulfilled.", body = ErrorMessage),
        (status = 404, description = "The requested resource does not exist.", body = ErrorMessage),
        (status = 500, description = "The request is not handled correctly due to a server error.", body = ErrorMessage),
    )
)]
pub async fn get(
    Extension(state): Extension<SharedState>,
    Path(AdminAccountsGetParams { account }): Path<AdminAccountsGetParams>,
) -> Result<Response, Error> {
    let Ok(account) = AccountName::new(account) else {
	return Err(Error::ValidationFailed);
    };
    let Ok(account) = AccountService::query_by_name(&account, &state.pg_pool).await else {
        return Err(anyhow!("error occured while querying account").into());
    };
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
        (status = 200, description = "The accounts were successfully returned.", body = AdminAccountsListResponse),
        (status = 400, description = "The request is malformed.", body = ErrorMessage),
        (status = 401, description = "The request is unauthenticated. The bearer token is missing or incorrect.", body = ErrorMessage),
        (status = 403, description = "The request is forbidden from being fulfilled.", body = ErrorMessage),
        (status = 500, description = "The request is not handled correctly due to a server error.", body = ErrorMessage),
    )
)]
pub async fn list(
    Extension(state): Extension<SharedState>,
    Query(AdminAccountsListQuery {
        max_results,
        page_token,
    }): Query<AdminAccountsListQuery>,
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
        let Ok(after) = AccountName::new(name) else {
	    return Err(Error::ValidationFailed);
	};
        Some(after)
    } else {
        None
    };
    let Ok(accounts) = AccountService::query(Some(&((limit + 1) as i64)), after.as_ref(), &state.pg_pool).await else {
        return Err(anyhow!("error occured while querying account(s)").into());
    };
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
