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
use anyhow::anyhow;
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
    let Ok(account) = AccountName::new(account) else {
        return Err(Error::ValidationFailed);
    };
    let Ok(account) = AccountEntity::load(&account, &state.pg_pool).await else {
        return Err(anyhow!("error occured while selecting account from database").into());
    };
    let Some(account) = account else {
        return Err(Error::Unauthorized);
    };
    let Ok(_) = account.verify(password.as_bytes()) else {
        return Err(Error::Unauthorized);
    };
    let Ok(profile) = ProfileService::issue(
        account.name().to_string(),
        account.email().to_string(),
        account.namespace().to_string(),
        Role::Admin,
        account.ttl().to_i64(),
    ) else {
        return Err(anyhow!("failed to create profile").into());
    };
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
