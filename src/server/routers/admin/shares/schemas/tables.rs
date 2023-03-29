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
use utoipa::IntoParams;
use utoipa::ToSchema;

#[derive(Debug, serde::Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct AdminSharesSchemasTablesPostParams {
    share: String,
    schema: String,
}

#[derive(Debug, serde::Deserialize, ToSchema)]
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
#[tracing::instrument(skip(state, account))]
pub async fn post(
    Extension(account): Extension<AccountEntity>,
    Extension(state): Extension<SharedState>,
    Path(params): Path<AdminSharesSchemasTablesPostParams>,
    Json(payload): Json<AdminSharesSchemasTablesPostRequest>,
) -> Result<Response, Error> {
    let Ok(share) = ShareName::new(params.share) else {
        tracing::error!("requested share data is malformed");
	return Err(Error::ValidationFailed);
    };
    let Ok(share) = ShareEntity::load(&share, &state.pg_pool).await else {
        tracing::error!("request is not handled correctly due to a server error while selecting share");
        return Err(anyhow!("error occured while selecting share").into());
    };
    let Some(share) = share else {
        tracing::error!("share was not found");
	return Err(Error::BadRequest);
    };
    let Ok(table) = TableName::new(payload.table) else {
        tracing::error!("requested table data is malformed");
	return Err(Error::ValidationFailed);
    };
    let Ok(table) = TableEntity::load(&table, &state.pg_pool).await else {
        tracing::error!("request is not handled correctly due to a server error while selecting table");
        return Err(anyhow!("error occured while selecting table").into());
    };
    let Some(table) = table else {
        tracing::error!("table was not found");
	return Err(Error::BadRequest);
    };
    let Ok(schema) = SchemaEntity::new(
        payload.id,
        params.schema,
        table.id().to_string(),
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
                Json(AdminSharesSchemasTablesPostResponse {
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
