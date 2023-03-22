use crate::error::Error;
use crate::server::entities::account::Entity as Account;
use crate::server::entities::account::Name as AccountName;
use crate::server::interactors::SharedState;
use crate::server::services::sharing::Service as SharingService;
use crate::utils::jwt::Claims;
use crate::utils::jwt::Role;
use crate::utils::postgres::has_conflict;
use crate::utils::postgres::pg_error;
use anyhow::anyhow;
use axum::extract::Extension;
use axum::extract::Json;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use tracing::error;
use tracing::info;
use tracing::warn;

const DEFAULT_PAGE_RESULTS: usize = 10;

#[derive(serde::Deserialize)]
pub struct LoginJson {
    name: String,
    password: String,
}

#[derive(serde::Deserialize)]
pub struct RegisterJson {
    id: Option<String>,
    name: String,
    email: String,
    password: String,
    namespace: String,
    ttl: i64,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountsQuery {
    max_results: Option<usize>,
    page_token: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountsPage {
    pub items: Vec<Account>,
    pub next_page_token: String,
}

pub async fn login(
    Extension(state): Extension<SharedState>,
    Json(payload): Json<LoginJson>,
) -> Result<Response, Error> {
    let name = if let Ok(name) = AccountName::new(payload.name) {
        name
    } else {
        error!("failed to validate admin account name");
        return Err(Error::ValidationFailed);
    };
    let admin = if let Some(admin) = Account::find_by_name(&name, &state.pg_pool).await? {
        admin
    } else {
        warn!("failed to authorize admin account");
        return Err(Error::Unauthorized);
    };
    if let Err(_) = admin.verify(payload.password.as_bytes()) {
        warn!("password did not match");
        return Err(Error::Unauthorized);
    }
    let profile = if let Ok(profile) = SharingService::profile_v1(
        admin.name().to_string(),
        admin.email().to_string(),
        admin.namespace().to_string(),
        Role::Admin,
        admin.ttl().to_i64(),
    ) {
        profile
    } else {
        error!("failed to create sharing profile");
        return Err(anyhow!("Profile creation failed").into());
    };
    Ok((StatusCode::OK, Json(profile)).into_response())
}

pub async fn register(
    _claims: Claims,
    Extension(state): Extension<SharedState>,
    Json(payload): Json<RegisterJson>,
) -> Result<Response, Error> {
    let account = if let Ok(account) = Account::new(
        payload.id,
        payload.name,
        payload.email,
        payload.password,
        payload.namespace,
        payload.ttl,
    ) {
        account
    } else {
        error!("failed to validate new admin account");
        return Err(Error::ValidationFailed);
    };
    match pg_error(account.register(&state.pg_pool).await)? {
        Ok(_) => {
            info!(
                r#"updated admin account id: "{}" name: "{}""#,
                account.id().as_uuid(),
                account.name().as_str()
            );
            Ok((StatusCode::CREATED, Json(account)).into_response())
        }
        Err(e) if has_conflict(&e) => {
            warn!("failed to update admin account: {}", e);
            Err(Error::Conflict)
        }
        _ => Err(anyhow!("Unknown error").into()),
    }
}

pub async fn accounts(
    _claims: Claims,
    Extension(state): Extension<SharedState>,
    query: Query<AccountsQuery>,
) -> Result<Response, Error> {
    let limit = query.max_results.unwrap_or(DEFAULT_PAGE_RESULTS);
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
    let accounts = Account::list(&((limit + 1) as i64), &after, &state.pg_pool).await?;
    if accounts.len() == limit + 1 {
        let last = &accounts[limit];
        let accounts = &accounts[..limit];
        return Ok((
            StatusCode::OK,
            Json(AccountsPage {
                items: accounts.to_vec(),
                next_page_token: last.name().to_string(),
            }),
        )
            .into_response());
    }
    Ok((
        StatusCode::OK,
        Json(AccountsPage {
            items: accounts,
            next_page_token: None.unwrap_or_default(),
        }),
    )
        .into_response())
}
