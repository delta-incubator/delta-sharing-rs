use crate::server::entities::account::Entity as AccountEntity;
use crate::server::entities::schema::Entity as SchemaEntity;
use crate::server::entities::share::Entity as ShareEntity;
use crate::server::entities::share::Name as ShareName;
use crate::server::entities::table::Entity as TableEntity;
use crate::server::entities::table::Name as TableName;
use crate::server::routers::SharedState;
use crate::server::services::error::Error;
use crate::server::services::schema::Schema;
use crate::server::utilities::postgres::Utility as PostgresUtility;
use anyhow::anyhow;
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
    pub table: String,
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
        (status = 201, description = "The schema was successfully registered.", body = AdminSharesSchemasTablesPostResponse),
        (status = 400, description = "The request is malformed.", body = ErrorMessage),
        (status = 401, description = "The request is unauthenticated. The bearer token is missing or incorrect.", body = ErrorMessage),
        (status = 409, description = "The schema was already registered.", body = ErrorMessage),
        (status = 500, description = "The request is not handled correctly due to a server error.", body = ErrorMessage),
    )
)]
pub async fn post(
    Extension(account): Extension<AccountEntity>,
    Extension(state): Extension<SharedState>,
    Path(AdminSharesSchemasTablesPostParams { share, schema }): Path<
        AdminSharesSchemasTablesPostParams,
    >,
    Json(AdminSharesSchemasTablesPostRequest { id, table }): Json<
        AdminSharesSchemasTablesPostRequest,
    >,
) -> Result<Response, Error> {
    let Ok(share) = ShareName::new(share) else {
	return Err(Error::ValidationFailed);
    };
    let Ok(share) = ShareEntity::load(&share, &state.pg_pool).await else {
        return Err(anyhow!("error occured while selecting share").into());
    };
    let Some(share) = share else {
	return Err(Error::BadRequest);
    };
    let Ok(table) = TableName::new(table) else {
	return Err(Error::ValidationFailed);
    };
    let Ok(table) = TableEntity::load(&table, &state.pg_pool).await else {
        return Err(anyhow!("error occured while selecting table").into());
    };
    let Some(table) = table else {
	return Err(Error::BadRequest);
    };
    let Ok(schema) = SchemaEntity::new(
        id,
        schema,
        table.id().to_string(),
        share.id().to_string(),
        account.id().to_string(),
    ) else {
	return Err(Error::ValidationFailed);
    };
    match PostgresUtility::error(schema.save(&state.pg_pool).await)? {
        Ok(_) => {
            debug!(
                r#"updated schema id: "{}" name: "{}""#,
                schema.id().as_uuid(),
                schema.name().as_str()
            );
            Ok((
                StatusCode::CREATED,
                Json(AdminSharesSchemasTablesPostResponse {
                    schema: Schema::from(schema),
                }),
            )
                .into_response())
        }
        Err(e) if PostgresUtility::is_conflict(&e) => Err(Error::Conflict),
        _ => Err(anyhow!("error occured while updating schema").into()),
    }
}
