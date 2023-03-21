use crate::config::JWT_SECRET;
use crate::error::Error;
use crate::server::entities::account::Entity as Account;
use crate::server::entities::account::Name as AccountName;
use crate::server::interactors::SharedState;
use crate::utils::jwt::expires_at;
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
use jsonwebtoken::encode;
use jsonwebtoken::Header;
use serde_json::json;
use tracing::error;
use tracing::info;
use tracing::warn;

#[derive(serde::Deserialize)]
pub struct RegisterJson {
    id: Option<String>,
    name: String,
    email: String,
    password: String,
    namespace: String,
}

#[derive(serde::Deserialize)]
pub struct LoginJson {
    name: String,
    password: String,
}

#[derive(serde::Deserialize)]
pub struct ListQuery {
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
    let expiry = if let Ok(expiry) = expires_at() {
        expiry
    } else {
        error!("failed to create JWT expiry");
        return Err(anyhow!("Setting expiration date in the past").into());
    };
    let claims = Claims {
        name: admin.name().to_string(),
        email: admin.email().to_string(),
        namespace: admin.namespace().to_string(),
        role: Role::Admin,
        exp: expiry,
    };
    let token = if let Ok(token) = encode(&Header::default(), &claims, &JWT_SECRET.encoding) {
        token
    } else {
        error!("failed to create JWT token");
        return Err(anyhow!("JWT creation failed").into());
    };
    Ok((
        StatusCode::OK,
        Json(json!({ "access_token": token, "type": "Bearer" })),
    )
        .into_response())
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

pub async fn list(
    _claims: Claims,
    Extension(state): Extension<SharedState>,
    query: Query<ListQuery>,
) -> Result<Response, Error> {
    let limit = query.results.unwrap_or(10);
    let page = query.page.unwrap_or(0);
    let offset = limit * page;
    let accounts = Account::list(&limit, &offset, &state.pg_pool).await?;
    Ok((StatusCode::OK, Json(accounts)).into_response())
}
