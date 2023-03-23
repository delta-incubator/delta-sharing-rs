use crate::config;
use crate::error::Error;
use crate::protos::protocol::Account;
use crate::protos::protocol::AdminAccountsRequest;
use crate::protos::protocol::AdminAccountsResponse;
use crate::protos::protocol::AdminLoginRequest;
use crate::protos::protocol::AdminRegisterRequest;
use crate::protos::protocol::Profile;
use crate::server::entities::account::Entity as AccountEntity;
use crate::server::entities::account::Name as AccountName;
use crate::server::interactors::SharedState;
use crate::server::services::sharing::Service as SharingService;
use crate::server::services::sharing::VERSION as SHARE_CREDENTIALS_VERSION;
use crate::utils::jwt::expires_in;
use crate::utils::jwt::Claims;
use crate::utils::jwt::Role;
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

const DEFAULT_PAGE_RESULTS: usize = 10;

pub async fn login(
    Extension(state): Extension<SharedState>,
    Json(payload): Json<AdminLoginRequest>,
) -> Result<Response, Error> {
    let name = if let Ok(name) = AccountName::new(payload.name) {
        name
    } else {
        error!("failed to validate admin account name");
        return Err(Error::ValidationFailed);
    };
    let entity = if let Some(entity) = AccountEntity::find_by_name(&name, &state.pg_pool).await? {
        entity
    } else {
        warn!("failed to authorize admin account");
        return Err(Error::Unauthorized);
    };
    if let Err(_) = entity.verify(payload.password.as_bytes()) {
        warn!("password did not match");
        return Err(Error::Unauthorized);
    }
    let (expiry, expiration_time) =
        if let Ok((expiry, expiration_time)) = expires_in(entity.ttl().to_i64()) {
            (expiry, expiration_time)
        } else {
            error!("failed to calculate expiration time");
            return Err(anyhow!("Expiration time calculation failed").into());
        };
    let token = if let Ok(token) = SharingService::token(
        entity.name().to_string(),
        entity.email().to_string(),
        entity.namespace().to_string(),
        Role::Admin,
        expiry,
    ) {
        token
    } else {
        error!("failed to create sharing bearer token");
        return Err(anyhow!("Profile creation failed").into());
    };
    let mut profile = Profile::new();
    profile.share_credentials_version = SHARE_CREDENTIALS_VERSION;
    profile.endpoint = config::fetch::<String>("server_bind");
    profile.bearer_token = token;
    profile.expiration_time = expiration_time.to_string();
    Ok((StatusCode::OK, Json(profile)).into_response())
}

pub async fn register(
    _claims: Claims,
    Extension(state): Extension<SharedState>,
    Json(payload): Json<AdminRegisterRequest>,
) -> Result<Response, Error> {
    let entity = if let Ok(entity) = AccountEntity::new(
        payload.id,
        payload.name,
        payload.email,
        payload.password,
        payload.namespace,
        payload.ttl,
    ) {
        entity
    } else {
        error!("failed to validate new admin account");
        return Err(Error::ValidationFailed);
    };
    match pg_error(entity.register(&state.pg_pool).await)? {
        Ok(_) => {
            info!(
                r#"updated admin account id: "{}" name: "{}""#,
                entity.id().as_uuid(),
                entity.name().as_str()
            );
            let mut account = Account::new();
            account.name = entity.name().to_string();
            account.email = entity.email().to_string();
            account.namespace = entity.namespace().to_string();
            account.ttl = entity.ttl().to_i64();
            Ok((StatusCode::CREATED, Json(account)).into_response())
        }
        Err(e) if has_conflict(&e) => {
            warn!("failed to update admin account: {}", e);
            Err(Error::Conflict)
        }
        _ => Err(anyhow!("Unknown error").into()),
    }
}

pub async fn accounts(
    _claims: Claims,
    Extension(state): Extension<SharedState>,
    query: Query<AdminAccountsRequest>,
) -> Result<Response, Error> {
    let limit = if let Some(limit) = &query.max_results {
        if let Ok(limit) = usize::try_from(*limit) {
            limit
        } else {
            error!("failed to validate max results query");
            return Err(Error::ValidationFailed);
        }
    } else {
        DEFAULT_PAGE_RESULTS
    };
    let after = if let Some(name) = &query.page_token {
        if let Ok(name) = AccountName::new(name) {
            Some(name)
        } else {
            error!("failed to validate account name");
            return Err(Error::ValidationFailed);
        }
    } else {
        None
    };
    let entities = AccountEntity::list(&((limit + 1) as i64), &after, &state.pg_pool).await?;
    if entities.len() == limit + 1 {
        let next = &entities[limit];
        let entities = &entities[..limit];
        let mut accounts = AdminAccountsResponse::new();
        accounts.items = entities
            .iter()
            .map(|entity| {
                let mut account = Account::new();
                account.name = entity.name().to_string();
                account.email = entity.email().to_string();
                account.namespace = entity.namespace().to_string();
                account.ttl = entity.ttl().to_i64();
                account
            })
            .collect();
        accounts.next_page_token = next.name().to_string();
        return Ok((StatusCode::OK, Json(accounts)).into_response());
    }
    let mut accounts = AdminAccountsResponse::new();
    accounts.items = entities
        .iter()
        .map(|entity| {
            let mut account = Account::new();
            account.name = entity.name().to_string();
            account.email = entity.email().to_string();
            account.namespace = entity.namespace().to_string();
            account.ttl = entity.ttl().to_i64();
            account
        })
        .collect();
    accounts.next_page_token = None.unwrap_or_default();
    Ok((StatusCode::OK, Json(accounts)).into_response())
}
