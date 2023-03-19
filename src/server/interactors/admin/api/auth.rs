use crate::config::JWT_SECRET;
use crate::error::Error;
use crate::server::entities::account::Account;
use crate::server::entities::account::AccountName;
use crate::server::interactors::SharedState;
use crate::utils::jwt::expires_at;
use crate::utils::jwt::Claims;
use crate::utils::postgres::has_conflict;
use crate::utils::postgres::pg_error;
use anyhow::anyhow;
use axum::extract::Extension;
use axum::extract::Json;
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

pub async fn register(
    claims: Claims,
    Extension(state): Extension<SharedState>,
    Json(payload): Json<RegisterJson>,
) -> Result<Response, Error> {
    let name = if let Ok(name) = AccountName::new(claims.name) {
        name
    } else {
        error!("failed to validate admin account name");
        return Err(Error::ValidationFailed);
    };
    if None == Account::find_by_name(&name, &state.pg_pool).await? {
        warn!("failed to authorize admin account");
        return Err(Error::Unauthorized);
    };
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
            Ok((
                StatusCode::CREATED,
                Json(json!({
                    "name": account.name(),
                    "email": account.email(),
                    "namespace": account.namespace()
                })),
            )
                .into_response())
        }
        Err(e) if has_conflict(&e) => {
            warn!("failed to update admin account: {}", e);
            Err(Error::Conflict)
        }
        _ => Err(anyhow!("Unknown error").into()),
    }
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
