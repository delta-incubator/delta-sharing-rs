pub mod accounts;
pub mod shares;
pub mod tables;
use crate::config;
use crate::server::entities::account::Entity as AccountEntity;
use crate::server::entities::account::Name as AccountName;
use crate::server::error::Error;
use crate::server::routers::SharedState;
use crate::server::schemas::claims::Role;
use crate::server::schemas::profile::Profile;
use crate::server::utilities::sharing::Utility as SharingUtility;
use crate::server::utilities::sharing::VERSION as SHARING_VERSION;
use anyhow::Context;
use axum::extract::Extension;
use axum::extract::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use tracing::debug;
use utoipa::ToSchema;

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
        (status = 401, description = "Authorization failed", body = Error),
        (status = 422, description = "Validation failed", body = Error),
        (status = 500, description = "Expiration time calculation and/or profile creation failed", body = Error),
    )
)]
pub async fn login(
    Extension(state): Extension<SharedState>,
    Json(payload): Json<AdminLoginRequest>,
) -> Result<Response, Error> {
    let name = AccountName::new(payload.name).map_err(|_| Error::ValidationFailed)?;
    let entity = AccountEntity::load(&name, &state.pg_pool)
        .await
        .context("error occured while selecting account from database")?;
    let Some(entity) = entity else {
        return Err(Error::Unauthorized);
    };
    entity
        .verify(payload.password.as_bytes())
        .map_err(|_| Error::Unauthorized)?;
    let (expiration_secs, expiration_time) = SharingUtility::new_expiration(entity.ttl().to_i64())
        .context("expiration time calculation failed")?;
    let token = SharingUtility::new_token(
        entity.name().to_string(),
        entity.email().to_string(),
        entity.namespace().to_string(),
        Role::Admin,
        expiration_secs,
    )
    .context("profile creation failed")?;
    debug!(
        r#"logged-in successfully id: "{}" name: "{}""#,
        entity.id().as_uuid(),
        entity.name().as_str()
    );
    Ok((
        StatusCode::OK,
        Json(AdminLoginResponse {
            profile: Profile {
                share_credentials_version: SHARING_VERSION,
                endpoint: config::fetch::<String>("server_bind"),
                bearer_token: token,
                expiration_time: expiration_time.to_string(),
            },
        }),
    )
        .into_response())
}
