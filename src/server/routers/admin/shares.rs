pub mod schemas;
use crate::server::entities::account::Entity as AccountEntity;
use crate::server::entities::share::Entity as ShareEntity;
use crate::server::routers::SharedState;
use crate::server::services::error::Error;
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
        (status = 201, description = "The share was successfully registered.", body = AdminSharesPostResponse),
        (status = 400, description = "The request is malformed.", body = ErrorMessage),
        (status = 401, description = "The request is unauthenticated. The bearer token is missing or incorrect.", body = ErrorMessage),
        (status = 409, description = "The share was already registered.", body = ErrorMessage),
        (status = 500, description = "The request is not handled correctly due to a server error.", body = ErrorMessage),
    )
)]
pub async fn post(
    Extension(account): Extension<AccountEntity>,
    Extension(state): Extension<SharedState>,
    Json(AdminSharesPostRequest { id, name }): Json<AdminSharesPostRequest>,
) -> Result<Response, Error> {
    let Ok(share) = ShareEntity::new(id, name, account.id().to_string()) else {
        return Err(Error::ValidationFailed);
    };
    match PostgresUtility::error(share.save(&state.pg_pool).await)? {
        Ok(_) => {
            debug!(
                r#"updated share id: "{}" name: "{}""#,
                share.id().as_uuid(),
                share.name().as_str()
            );
            Ok((
                StatusCode::CREATED,
                Json(AdminSharesPostResponse {
                    share: Share::from(share),
                }),
            )
                .into_response())
        }
        Err(e) if PostgresUtility::is_conflict(&e) => Err(Error::Conflict),
        _ => Err(anyhow!("error occured while updating share").into()),
    }
}
