pub mod accounts;
pub mod shares;
use crate::config;
use crate::error::Error;
use crate::server::entities::account::Entity as AccountEntity;
use crate::server::entities::account::Name as AccountName;
use crate::server::interactors::SharedState;
use crate::server::services::sharing::Service as SharingService;
use crate::server::services::sharing::VERSION as SHARE_CREDENTIALS_VERSION;
use crate::utils::jwt::expires_in;
use crate::utils::jwt::Role;
use anyhow::anyhow;
use axum::extract::Extension;
use axum::extract::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use tracing::error;
use tracing::info;
use tracing::warn;
use utoipa::ToSchema;

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub share_credentials_version: i64,
    pub endpoint: String,
    pub bearer_token: String,
    pub expiration_time: String,
}

#[derive(serde::Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdminLoginRequest {
    pub name: String,
    pub password: String,
}

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdminLoginResponse {
    pub profile: Profile,
}

#[utoipa::path(
    post,
    path = "/admin/login",
    request_body = AdminLoginRequest,
    responses(
        (status = 200, description = "Logged-in successfully", body = AdminLoginResponse),
        (status = 401, description = "Authorization failed", body = ErrorResponse),
        (status = 422, description = "Validation failed", body = ErrorResponse),
        (status = 500, description = "Expiration time calculation and/or profile creation failed", body = ErrorResponse),
    )
)]
pub async fn login(
    Extension(state): Extension<SharedState>,
    Json(payload): Json<AdminLoginRequest>,
) -> Result<Response, Error> {
    let name = if let Ok(name) = AccountName::new(payload.name) {
        name
    } else {
        error!("failed to validate account name");
        return Err(Error::ValidationFailed);
    };
    let entity = if let Some(entity) = AccountEntity::find_by_name(&name, &state.pg_pool).await? {
        entity
    } else {
        warn!("failed to authorize account");
        return Err(Error::Unauthorized);
    };
    if let Err(_) = entity.verify(payload.password.as_bytes()) {
        warn!("password did not match");
        return Err(Error::Unauthorized);
    }
    let (expiry, expiration_time) =
        if let Ok((expiry, expiration_time)) = expires_in(entity.ttl().to_i64()) {
            (expiry, expiration_time)
        } else {
            error!("failed to calculate expiration time");
            return Err(anyhow!("Expiration time calculation failed").into());
        };
    let token = if let Ok(token) = SharingService::token(
        entity.name().to_string(),
        entity.email().to_string(),
        entity.namespace().to_string(),
        Role::Admin,
        expiry,
    ) {
        token
    } else {
        error!("failed to create sharing bearer token");
        return Err(anyhow!("Profile creation failed").into());
    };
    info!(r#"account "{}" logged in"#, entity.name().as_str());
    Ok((
        StatusCode::OK,
        Json(AdminLoginResponse {
            profile: Profile {
                share_credentials_version: SHARE_CREDENTIALS_VERSION,
                endpoint: config::fetch::<String>("server_bind"),
                bearer_token: token,
                expiration_time: expiration_time.to_string(),
            },
        }),
    )
        .into_response())
}
