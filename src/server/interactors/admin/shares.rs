use crate::error::Error;
use crate::server::entities::account::Entity as AccountEntity;
use crate::server::entities::account::Name as AccountName;
use crate::server::entities::share::Entity as ShareEntity;
use crate::server::interactors::SharedState;
use crate::utils::jwt::Claims;
use crate::utils::postgres::has_conflict;
use crate::utils::postgres::pg_error;
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
pub struct Share {
    pub id: String,
    pub name: String,
}

#[derive(serde::Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdminSharesPostRequest {
    pub id: Option<String>,
    pub name: String,
}

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdminSharesPostResponse {
    pub share: Share,
}

#[utoipa::path(
    post,
    path = "/admin/shares",
    request_body = AdminSharesPostRequest,
    responses(
        (status = 201, description = "Registered share successfully", body = AdminSharesPostResponse),
        (status = 401, description = "Authorization failed", body = ErrorResponse),
        (status = 409, description = "Confliction occured", body = ErrorResponse),
        (status = 422, description = "Validation failed", body = ErrorResponse),
        (status = 500, description = "Error occured while creating share on database", body = ErrorResponse),
    )
)]
pub async fn post(
    claims: Claims,
    Extension(state): Extension<SharedState>,
    Json(payload): Json<AdminSharesPostRequest>,
) -> Result<Response, Error> {
    let account = if let Ok(account) = AccountName::new(claims.name) {
        account
    } else {
        error!("failed to validate account name");
        return Err(Error::ValidationFailed);
    };
    let account =
        if let Some(account) = AccountEntity::find_by_name(&account, &state.pg_pool).await? {
            account
        } else {
            warn!("failed to authorize account");
            return Err(Error::Unauthorized);
        };
    let entity =
        if let Ok(entity) = ShareEntity::new(payload.id, payload.name, account.id().to_string()) {
            entity
        } else {
            error!("failed to validate new share");
            return Err(Error::ValidationFailed);
        };
    match pg_error(entity.register(&state.pg_pool).await)? {
        Ok(_) => {
            info!(
                r#"updated share id: "{}" name: "{}""#,
                entity.id().as_uuid(),
                entity.name().as_str()
            );
            Ok((
                StatusCode::CREATED,
                Json(AdminSharesPostResponse {
                    share: Share {
                        id: entity.id().to_string(),
                        name: entity.name().to_string(),
                    },
                }),
            )
                .into_response())
        }
        Err(e) if has_conflict(&e) => {
            warn!("failed to update share: {}", e);
            Err(Error::Conflict)
        }
        _ => Err(anyhow!("Unknown error").into()),
    }
}
