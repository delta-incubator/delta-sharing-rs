use crate::infra::postgres::has_conflict;
use crate::infra::postgres::pg_error;
use crate::server::entities::account::Account;
use crate::server::entities::account::AccountId;
use crate::server::interactors::InteractorError;
use crate::server::interactors::SharedState;
use crate::server::services::account::AccountService;
use anyhow::anyhow;
use axum::extract::Extension;
use axum::extract::Json;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use tracing::error;
use tracing::info;
use tracing::warn;

#[derive(serde::Deserialize)]
pub struct CreateJson {
    id: Option<String>,
    name: String,
    email: String,
    password: String,
    namespace: String,
}

pub async fn create(
    Extension(state): Extension<SharedState>,
    Json(payload): Json<CreateJson>,
) -> Result<Response, InteractorError> {
    let id = payload.id.unwrap_or(uuid::Uuid::new_v4().to_string());
    let account = if let Ok(account) = Account::new(
        id,
        payload.name,
        payload.email,
        payload.password,
        payload.namespace,
    ) {
        account
    } else {
        error!("invalid account specification found");
        return Err(InteractorError::ValidationFailed);
    };
    match pg_error(AccountService::create(&state.server.db_pool, &account).await)? {
        Ok(_) => {
            info!(
                r#"updated account id: "{}" name: "{}""#,
                account.id().as_uuid(),
                account.name().as_str()
            );
            Ok((StatusCode::CREATED, Json(account)).into_response())
        }
        Err(e) if has_conflict(&e) => {
            warn!("failed to update account: {}", e);
            Err(InteractorError::Conflict)
        }
        _ => Err(InteractorError::InternalServerProblem(anyhow!(
            "Internal server error"
        ))),
    }
}

pub async fn delete(
    Extension(state): Extension<SharedState>,
    Path(id): Path<String>,
) -> Result<Response, InteractorError> {
    let id = if let Ok(id) = AccountId::try_from(id) {
        id
    } else {
        error!("account id must be uuid v4");
        return Err(InteractorError::BadRequest);
    };
    match pg_error(AccountService::delete(&state.server.db_pool, &id).await)? {
        Ok(done) => {
            if done.rows_affected() == 1 {
                info!(r#"deleted account id: "{}""#, id.as_uuid());
                Ok(StatusCode::NO_CONTENT.into_response())
            } else {
                info!(r#"no account was found with id: "{}""#, id.as_uuid());
                Ok(StatusCode::NOT_FOUND.into_response())
            }
        }
        Err(e) => {
            warn!("failed to delete account: {}", e);
            Err(InteractorError::InternalServerProblem(anyhow!(
                "Internal server error"
            )))
        }
    }
}
