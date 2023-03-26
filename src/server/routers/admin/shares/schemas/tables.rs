use crate::server::entities::account::Entity as AccountEntity;
use crate::server::entities::schema::Entity as SchemaEntity;
use crate::server::entities::share::Entity as ShareEntity;
use crate::server::entities::share::Name as ShareName;
use crate::server::entities::table::Entity as TableEntity;
use crate::server::entities::table::Name as TableName;
use crate::server::error::Error;
use crate::server::routers::SharedState;
use crate::server::schemas::schema::Schema;
use crate::server::utilities::postgres::Utility as PostgresUtility;
use anyhow::anyhow;
use anyhow::Context;
use axum::extract::Extension;
use axum::extract::Json;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use tracing::debug;
use utoipa::IntoParams;
use utoipa::ToSchema;

#[derive(serde::Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct AdminSharesSchemasTablesPostParams {
    share: String,
    schema: String,
}

#[derive(serde::Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdminSharesSchemasTablesPostRequest {
    pub id: Option<String>,
    pub name: String,
}

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdminSharesSchemasTablesPostResponse {
    pub schema: Schema,
}

#[utoipa::path(
    post,
    path = "admin/shares/{share}/schemas/{schema}/tables",
    params(
        AdminSharesSchemasTablesPostParams,
    ),
    request_body = AdminSharesSchemasTablesPostRequest,
    responses(
        (status = 201, description = "Registered schema successfully", body = AdminSharesSchemasTablesPostResponse),
        (status = 400, description = "Requested table was not found", body = Error),
        (status = 401, description = "Authorization failed", body = Error),
        (status = 409, description = "Confliction occured", body = Error),
        (status = 422, description = "Validation failed", body = Error),
        (status = 500, description = "Error occured while creating schema on database", body = Error),
    )
)]
pub async fn post(
    Extension(account): Extension<AccountEntity>,
    Extension(state): Extension<SharedState>,
    Path(AdminSharesSchemasTablesPostParams { share, schema }): Path<
        AdminSharesSchemasTablesPostParams,
    >,
    Json(payload): Json<AdminSharesSchemasTablesPostRequest>,
) -> Result<Response, Error> {
    let share = ShareName::new(share).map_err(|_| Error::ValidationFailed)?;
    let share = ShareEntity::load(&share, &state.pg_pool)
        .await
        .context("error occured while selecting share")?;
    let Some(share) = share else {
	return Err(Error::NotFound);
    };
    let table = TableName::new(payload.name).map_err(|_| Error::ValidationFailed)?;
    let table = TableEntity::find_by_name(&table, &state.pg_pool)
        .await
        .context("error occured while selecting table")?;
    let Some(table) = table else {
	return Err(Error::BadRequest);
    };
    let entity = SchemaEntity::new(
        payload.id,
        schema,
        table.id().to_string(),
        share.id().to_string(),
        account.id().to_string(),
    )
    .map_err(|_| Error::ValidationFailed)?;
    match PostgresUtility::error(entity.register(&state.pg_pool).await)? {
        Ok(_) => {
            debug!(
                r#"updated schema id: "{}" name: "{}""#,
                entity.id().as_uuid(),
                entity.name().as_str()
            );
            Ok((
                StatusCode::CREATED,
                Json(AdminSharesSchemasTablesPostResponse {
                    schema: Schema {
                        id: entity.id().to_string(),
                        name: entity.name().to_string(),
                    },
                }),
            )
                .into_response())
        }
        Err(e) if PostgresUtility::is_conflict(&e) => Err(Error::Conflict),
        _ => Err(anyhow!("error occured while updating schema").into()),
    }
}
