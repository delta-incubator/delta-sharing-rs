use crate::server::entities::account::Entity as AccountEntity;
use crate::server::entities::table::Entity as TableEntity;
use crate::server::entities::table::Name as TableName;
use crate::server::error::Error;
use crate::server::routers::SharedState;
use crate::server::services::table::Service as TableService;
use crate::server::services::table::Table;
use crate::server::utilities::postgres::Utility as PostgresUtility;
use anyhow::anyhow;
use anyhow::Context;
use axum::extract::Extension;
use axum::extract::Json;
use axum::extract::Path;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use tracing::debug;
use utoipa::IntoParams;
use utoipa::ToSchema;

const DEFAULT_PAGE_RESULTS: usize = 10;

#[derive(serde::Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdminTablesPostRequest {
    pub id: Option<String>,
    pub name: String,
    pub location: String,
}

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdminTablesPostResponse {
    pub table: Table,
}

#[utoipa::path(
    post,
    path = "/admin/tables",
    request_body = AdminTablesPostRequest,
    responses(
        (status = 201, description = "Registered table successfully", body = AdminTablesPostResponse),
        (status = 401, description = "Authorization failed", body = Error),
        (status = 409, description = "Confliction occured", body = Error),
        (status = 422, description = "Validation failed", body = Error),
        (status = 500, description = "Error occured while creating table on database", body = Error),
    )
)]
pub async fn post(
    Extension(account): Extension<AccountEntity>,
    Extension(state): Extension<SharedState>,
    Json(payload): Json<AdminTablesPostRequest>,
) -> Result<Response, Error> {
    let entity = TableEntity::new(
        payload.id,
        payload.name,
        payload.location,
        account.id().to_string(),
    )
    .map_err(|_| Error::ValidationFailed)?;
    match PostgresUtility::error(entity.save(&state.pg_pool).await)? {
        Ok(_) => {
            debug!(
                r#"updated table id: "{}" name: "{}""#,
                entity.id().as_uuid(),
                entity.name().as_str()
            );
            Ok((
                StatusCode::CREATED,
                Json(AdminTablesPostResponse {
                    table: Table::from(entity),
                }),
            )
                .into_response())
        }
        Err(e) if PostgresUtility::is_conflict(&e) => Err(Error::Conflict),
        _ => Err(anyhow!("error occured while updating table").into()),
    }
}

#[derive(serde::Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct AdminTablesGetParams {
    name: String,
}

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdminTablesGetResponse {
    pub table: Table,
}

#[utoipa::path(
    get,
    path = "/admin/tables/{name}",
    params(
        AdminTablesGetParams,
    ),
    responses(
        (status = 200, description = "Show matching table successfully", body = AdminTablesGetResponse),
        (status = 401, description = "Authorization failed", body = Error),
        (status = 404, description = "Table not found", body = Error),
        (status = 422, description = "Validation failed", body = Error),
        (status = 500, description = "Error occured while selecting table on database", body = Error),
    )
)]
pub async fn get(
    Extension(state): Extension<SharedState>,
    Path(AdminTablesGetParams { name }): Path<AdminTablesGetParams>,
) -> Result<Response, Error> {
    let name = TableName::new(name).map_err(|_| Error::ValidationFailed)?;
    let table = TableService::query_by_name(&name, &state.pg_pool)
        .await
        .context("error occured while selecting table")?;
    let Some(table) = table else {
	return Err(Error::NotFound);
    };
    debug!(r#"found table id: "{}" name: "{}""#, &table.id, &table.name);
    Ok((
        StatusCode::OK,
        Json(AdminTablesGetResponse { table: table }),
    )
        .into_response())
}

#[derive(serde::Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct AdminTablesListQuery {
    pub max_results: Option<i64>,
    pub page_token: Option<String>,
}

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdminTablesListResponse {
    pub items: Vec<Table>,
    pub next_page_token: String,
}

#[utoipa::path(
    get,
    path = "/admin/tables",
    params(
        AdminTablesListQuery,
    ),
    responses(
        (status = 200, description = "List matching table(s) successfbyully", body = AdminTablesListResponse),
        (status = 401, description = "Authorization failed", body = Error),
        (status = 422, description = "Validation failed", body = Error),
        (status = 500, description = "Error occured while selecting tables(s) on database", body = Error),
    )
)]
pub async fn list(
    Extension(state): Extension<SharedState>,
    query: Query<AdminTablesListQuery>,
) -> Result<Response, Error> {
    let limit = if let Some(limit) = &query.max_results {
        let limit = usize::try_from(*limit).map_err(|_| Error::ValidationFailed)?;
        limit
    } else {
        DEFAULT_PAGE_RESULTS
    };
    let after = if let Some(name) = &query.page_token {
        let after = TableName::new(name).map_err(|_| Error::ValidationFailed)?;
        Some(after)
    } else {
        None
    };
    let tables = TableService::query(Some(&((limit + 1) as i64)), after.as_ref(), &state.pg_pool)
        .await
        .context("error occured while selecting table(s)")?;
    if tables.len() == limit + 1 {
        let next = &tables[limit];
        let tables = &tables[..limit];
        debug!(r"found {} table(s)", tables.len());
        return Ok((
            StatusCode::OK,
            Json(AdminTablesListResponse {
                items: tables.to_vec(),
                next_page_token: next.name.clone(),
            }),
        )
            .into_response());
    }
    debug!(r"found {} table(s)", tables.len());
    Ok((
        StatusCode::OK,
        Json(AdminTablesListResponse {
            items: tables,
            next_page_token: None.unwrap_or_default(),
        }),
    )
        .into_response())
}
