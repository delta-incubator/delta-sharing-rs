pub mod schemas;
use crate::server::entities::account::Entity as AccountEntity;
use crate::server::entities::share::Entity as ShareEntity;
use crate::server::error::Error;
use crate::server::interactors::SharedState;
use crate::server::schemas::share::Share;
use crate::server::utils::postgres::has_conflict;
use crate::server::utils::postgres::pg_error;
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
    match pg_error(entity.register(&state.pg_pool).await)? {
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
                        id: entity.id().to_string(),
                        name: entity.name().to_string(),
                    },
                }),
            )
                .into_response())
        }
        Err(e) if has_conflict(&e) => Err(Error::Conflict),
        _ => Err(anyhow!("error occured while updating share").into()),
    }
}
