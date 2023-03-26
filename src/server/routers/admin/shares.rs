pub mod schemas;
use crate::server::entities::account::Entity as AccountEntity;
use crate::server::entities::share::Entity as ShareEntity;
use crate::server::error::Error;
use crate::server::routers::SharedState;
use crate::server::services::share::Share;
use crate::server::utilities::postgres::Utility as PostgresUtility;
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
        (status = 401, description = "Authorization failed", body = Error),
        (status = 409, description = "Confliction occured", body = Error),
        (status = 422, description = "Validation failed", body = Error),
        (status = 500, description = "Error occured while creating share on database", body = Error),
    )
)]
pub async fn post(
    Extension(account): Extension<AccountEntity>,
    Extension(state): Extension<SharedState>,
    Json(payload): Json<AdminSharesPostRequest>,
) -> Result<Response, Error> {
    let entity = ShareEntity::new(payload.id, payload.name, account.id().to_string())
        .map_err(|_| Error::ValidationFailed)?;
    match PostgresUtility::error(entity.save(&state.pg_pool).await)? {
        Ok(_) => {
            debug!(
                r#"updated share id: "{}" name: "{}""#,
                entity.id().as_uuid(),
                entity.name().as_str()
            );
            Ok((
                StatusCode::CREATED,
                Json(AdminSharesPostResponse {
                    share: Share {
                        id: entity.id().to_uuid(),
                        name: entity.name().to_string(),
                    },
                }),
            )
                .into_response())
        }
        Err(e) if PostgresUtility::is_conflict(&e) => Err(Error::Conflict),
        _ => Err(anyhow!("error occured while updating share").into()),
    }
}
