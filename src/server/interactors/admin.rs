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
pub struct AccountsQuery {
    page: Option<i64>,
    results: Option<i64>,
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
    let limit = query.results.unwrap_or(10);
    let page = query.page.unwrap_or(0);
    let offset = limit * page;
    let accounts = Account::list(&limit, &offset, &state.pg_pool).await?;
    Ok((StatusCode::OK, Json(accounts)).into_response())
}
