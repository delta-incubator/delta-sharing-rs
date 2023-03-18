use crate::config::JWT_SECRET;
use crate::error::Error;
use crate::server::entities::account::Account;
use crate::server::entities::account::AccountName;
use crate::server::interactors::SharedState;
use crate::server::services::account::AccountService;
use crate::utils::argon2;
use crate::utils::jsonwebtoken::expires_at;
use crate::utils::jsonwebtoken::Claims;
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
    Extension(state): Extension<SharedState>,
    Json(payload): Json<RegisterJson>,
) -> Result<Response, Error> {
    let id = payload.id.unwrap_or(uuid::Uuid::new_v4().to_string());
    let account = if let Ok(account) = Account::new(
        id,
        payload.name,
        payload.email,
        argon2::hash(payload.password.as_bytes()).unwrap(),
        payload.namespace,
    ) {
        account
    } else {
        error!("failed to validate account");
        return Err(Error::ValidationFailed);
    };
    match pg_error(AccountService::create(&state.pg_pool, &account).await)? {
        Ok(_) => {
            info!(
                r#"updated account id: "{}" name: "{}""#,
                account.id().as_uuid(),
                account.name().as_str()
            );
            Ok((StatusCode::CREATED, Json(account)).into_response())
        }
        Err(e) if has_conflict(&e) => {
            warn!("failed to update account: {}", e);
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
        error!("failed to validate account name");
        return Err(Error::ValidationFailed);
    };
    match AccountService::get_by_name(&state.pg_pool, &name).await? {
        None => {
            warn!("failed to authorize account, no accuont found");
            return Err(Error::Unauthorized);
        }
        Some(row) => {
            if let Err(_) = argon2::verify(payload.password.as_bytes(), row.password.as_str()) {
                warn!("failed to authorize account, password did not match");
                return Err(Error::Unauthorized);
            }
            let expiry = if let Ok(expiry) = expires_at() {
                expiry
            } else {
                error!("failed to create JWT expiry");
                return Err(anyhow!("Setting expiration date in the past").into());
            };
            let claims = Claims {
                email: row.email.to_owned(),
                namespace: row.namespace.to_owned(),
                exp: expiry,
            };
            let token = if let Ok(token) = encode(&Header::default(), &claims, &JWT_SECRET.encoding)
            {
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
    }
}

pub async fn profile(claims: Claims) -> Result<Response, Error> {
    Ok((
        StatusCode::OK,
        Json(json!({ "email": claims.email, "namespace": claims.namespace })),
    )
        .into_response())
}
