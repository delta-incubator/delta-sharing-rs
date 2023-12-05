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
use crate::server::entities::table::Entity as TableEntity;
use crate::server::entities::table::Name as TableName;
use crate::server::routers::SharedState;
use crate::server::services::error::Error;
use crate::server::services::table::Table;
use crate::server::utilities::postgres::Utility as PostgresUtility;

#[derive(Debug, serde::Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct AdminSharesSchemasTablesPostParams {
    share: String,
    schema: String,
}

#[derive(Debug, serde::Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdminSharesSchemasTablesPostRequest {
    pub name: String,
    pub location: String,
}

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdminSharesSchemasTablesPostResponse {
    pub table: Table,
}

#[utoipa::path(
    post,
    path = "/admin/shares/{share}/schemas/{schema}/tables",
    operation_id = "CreateTable",
    tag = "admin",
    params(AdminSharesSchemasTablesPostParams),
    request_body = AdminSharesSchemasTablesPostRequest,
    responses(
        (status = 201, description = "The schema was successfully registered.", body = AdminSharesSchemasTablesPostResponse),
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
    Path(params): Path<AdminSharesSchemasTablesPostParams>,
    Json(payload): Json<AdminSharesSchemasTablesPostRequest>,
) -> Result<Response, Error> {
    let Ok(share_name) = ShareName::new(params.share) else {
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
    let Ok(schema_name) = SchemaName::new(params.schema) else {
        tracing::error!("requested share data is malformed");
        return Err(Error::ValidationFailed);
    };
    let Ok(maybe_schema) = SchemaEntity::load(share.id(), &schema_name, &state.pg_pool).await
    else {
        tracing::error!(
            "request is not handled correctly due to a server error while selecting share"
        );
        return Err(anyhow!("error occurred while selecting share").into());
    };
    let Some(schema) = maybe_schema else {
        tracing::error!("share was not found");
        return Err(Error::NotFound);
    };
    let Ok(table_name) = TableName::new(payload.name) else {
        tracing::error!("requested table data is malformed");
        return Err(Error::ValidationFailed);
    };
    let Ok(table) = TableEntity::new(
        None,
        table_name.to_string(),
        schema.id().to_string(),
        payload.location,
        account.id().to_string(),
    ) else {
        tracing::error!("requested schema data is malformed");
        return Err(Error::ValidationFailed);
    };
    match PostgresUtility::error(table.save(&state.pg_pool).await)? {
        Ok(_) => {
            tracing::info!("table was successfully registered");
            Ok((
                StatusCode::CREATED,
                Json(AdminSharesSchemasTablesPostResponse {
                    table: Table::from(table),
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
