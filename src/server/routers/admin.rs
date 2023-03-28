pub mod accounts;
pub mod shares;
pub mod tables;
use crate::server::entities::account::Entity as AccountEntity;
use crate::server::entities::account::Name as AccountName;
use crate::server::middlewares::jwt::Role;
use crate::server::routers::SharedState;
use crate::server::services::error::Error;
use crate::server::services::profile::Profile;
use crate::server::services::profile::Service as ProfileService;
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
    pub account: String,
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
        (status = 200, description = "The profile were successfbyully returned.", body = AdminLoginResponse),
        (status = 400, description = "The request is malformed.", body = ErrorMessage),
        (status = 401, description = "The request is unauthenticated.", body = ErrorMessage),
        (status = 500, description = "The request is not handled correctly due to a server error.", body = ErrorMessage),
    )
)]
pub async fn login(
    Extension(state): Extension<SharedState>,
    Json(AdminLoginRequest { account, password }): Json<AdminLoginRequest>,
) -> Result<Response, Error> {
    let account = AccountName::new(account).map_err(|_| Error::ValidationFailed)?;
    let account = AccountEntity::load(&account, &state.pg_pool)
        .await
        .context("error occured while selecting account from database")?;
    let Some(account) = account else {
        return Err(Error::Unauthorized);
    };
    account
        .verify(password.as_bytes())
        .map_err(|_| Error::Unauthorized)?;
    let profile = ProfileService::issue(
        account.name().to_string(),
        account.email().to_string(),
        account.namespace().to_string(),
        Role::Admin,
        account.ttl().to_i64(),
    )
    .context("failed to create profile")?;
    debug!(
        r#"logged-in successfully id: "{}" name: "{}""#,
        account.id().as_uuid(),
        account.name().as_str()
    );
    Ok((
        StatusCode::OK,
        Json(AdminLoginResponse { profile: profile }),
    )
        .into_response())
}
