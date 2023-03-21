use crate::error::Error;
use crate::server::entities::account::Entity as Account;
use crate::server::entities::account::Name as AccountName;
use crate::server::entities::share::Entity as Share;
use crate::server::interactors::SharedState;
use crate::utils::jwt::Claims;
use crate::utils::postgres::has_conflict;
use crate::utils::postgres::pg_error;
use anyhow::anyhow;
use axum::extract::Extension;
use axum::extract::Json;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use tracing::error;
use tracing::info;
use tracing::warn;

#[derive(serde::Deserialize)]
pub struct RegisterJson {
    id: Option<String>,
    name: String,
}

#[derive(serde::Deserialize)]
pub struct ListQuery {
    page: Option<i64>,
    results: Option<i64>,
}

pub async fn register(
    claims: Claims,
    Extension(state): Extension<SharedState>,
    Json(payload): Json<RegisterJson>,
) -> Result<Response, Error> {
    let name = if let Ok(name) = AccountName::new(claims.name) {
        name
    } else {
        error!("failed to validate admin account name");
        return Err(Error::ValidationFailed);
    };
    let admin = if let Some(admin) = Account::find_by_name(&name, &state.pg_pool).await? {
        admin
    } else {
        error!(
            r#"inconsistent JWT payload found, name: "{}" was not found on [account]"#,
            name.as_str()
        );
        return Err(anyhow!("Inconsistent JWT payload").into());
    };
    let share = if let Ok(share) = Share::new(payload.id, payload.name, admin.id().to_string()) {
        share
    } else {
        error!("failed to validate new share");
        return Err(Error::ValidationFailed);
    };
    match pg_error(share.register(&state.pg_pool).await)? {
        Ok(_) => {
            info!(
                r#"updated share id: "{}" name: "{}""#,
                share.id().as_uuid(),
                share.name().as_str()
            );
            Ok((StatusCode::CREATED, Json(share)).into_response())
        }
        Err(e) if has_conflict(&e) => {
            warn!("failed to update share: {}", e);
            Err(Error::Conflict)
        }
        _ => Err(anyhow!("Unknown error").into()),
    }
}

pub async fn list(
    claims: Claims,
    Extension(state): Extension<SharedState>,
    query: Query<ListQuery>,
) -> Result<Response, Error> {
    let name = if let Ok(name) = AccountName::new(claims.name) {
        name
    } else {
        error!("failed to validate admin account name");
        return Err(Error::ValidationFailed);
    };
    let limit = query.results.unwrap_or(10);
    let page = query.page.unwrap_or(0);
    let offset = limit * page;
    let shares = Share::list_by_account_name(&name, &limit, &offset, &state.pg_pool).await?;
    Ok((StatusCode::OK, Json(shares)).into_response())
}
