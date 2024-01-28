use anyhow::anyhow;
use axum::extract::Extension;
use axum::extract::Json;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use utoipa::IntoParams;
use utoipa::ToSchema;

use crate::server::entities::account::Entity as AccountEntity;
use crate::server::entities::schema::Entity as SchemaEntity;
use crate::server::entities::schema::Name as SchemaName;
use crate::server::entities::share::Entity as ShareEntity;
use crate::server::entities::share::Name as ShareName;
use crate::server::routers::SharedState;
use crate::server::services::error::Error;
use crate::server::services::schema::Schema;
use crate::server::utilities::postgres::Utility as PostgresUtility;

pub mod tables;

#[derive(Debug, serde::Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct AdminSharesSchemasPostParams {
    share: String,
}

#[derive(Debug, serde::Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdminSharesSchemasPostRequest {
    pub name: String,
}

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdminSharesSchemasPostResponse {
    pub schema: Schema,
}

#[utoipa::path(
    post,
    path = "/admin/shares/{share}/schemas",
    operation_id = "CreateSchema",
    tag = "admin",
    params(AdminSharesSchemasPostParams),
    request_body = AdminSharesSchemasPostRequest,
    responses(
        (status = 201, description = "The schema was successfully registered.", body = AdminSharesSchemasPostResponse),
        (status = 400, description = "The request is malformed.", body = ErrorMessage),
        (status = 401, description = "The request is unauthenticated. The bearer token is missing or incorrect.", body = ErrorMessage),
        (status = 409, description = "The schema was already registered.", body = ErrorMessage),
        (status = 500, description = "The request is not handled correctly due to a server error.", body = ErrorMessage),
    )
)]
#[tracing::instrument(skip(state, account))]
pub async fn post(
    Extension(account): Extension<AccountEntity>,
    Extension(state): Extension<SharedState>,
    Path(params): Path<AdminSharesSchemasPostParams>,
    Json(payload): Json<AdminSharesSchemasPostRequest>,
) -> Result<Response, Error> {
    let Ok(share_name) = ShareName::try_new(params.share) else {
        tracing::error!("requested share data is malformed");
        return Err(Error::ValidationFailed);
    };
    let Ok(maybe_share) = ShareEntity::load(&share_name, &state.pg_pool).await else {
        tracing::error!(
            "request is not handled correctly due to a server error while selecting share"
        );
        return Err(anyhow!("error occured while selecting share").into());
    };
    let Some(share) = maybe_share else {
        tracing::error!("share was not found");
        return Err(Error::NotFound);
    };
    let Ok(schema_name) = SchemaName::try_new(payload.name) else {
        tracing::error!("schema name is malformed");
        return Err(Error::ValidationFailed);
    };
    let Ok(schema) = SchemaEntity::new(
        None,
        schema_name.to_string(),
        share.id().to_string(),
        account.id().to_string(),
    ) else {
        tracing::error!("requested schema data is malformed");
        return Err(Error::ValidationFailed);
    };
    match PostgresUtility::error(schema.save(&state.pg_pool).await)? {
        Ok(_) => {
            tracing::info!("schema was successfully registered");
            Ok((
                StatusCode::CREATED,
                Json(AdminSharesSchemasPostResponse {
                    schema: Schema::from(schema),
                }),
            )
                .into_response())
        }
        Err(e) if PostgresUtility::is_conflict(&e) => {
            tracing::error!("schema was already registered");
            Err(Error::Conflict)
        }
        _ => {
            tracing::error!(
                "request is not handled correctly due to a server error while updating schema"
            );
            Err(anyhow!("error occured while updating schema").into())
        }
    }
}
