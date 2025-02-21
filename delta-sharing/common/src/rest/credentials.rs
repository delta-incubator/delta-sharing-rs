use axum::extract::{Extension, State};
use axum::routing::{delete, get, post};
use axum::{Json, Router};

use crate::api::{CredentialsHandler, RequestContext};
use crate::models::credentials::v1::*;
use crate::{Recipient, Result};

pub fn get_router<T: CredentialsHandler + Clone>(handler: T) -> Router {
    Router::new()
        .route("/credentials", post(create_credential::<T>))
        .route("/credentials/{name}", get(get_credential::<T>))
        // .route("/credentials/{name}", patch(update_credential::<T>))
        .route("/credentials/{name}", delete(delete_credential::<T>))
        .route("/storage_locations", post(create_storage_location::<T>))
        .route("/storage_locations", get(list_storage_locations::<T>))
        .route("/storage_locations/{name}", get(get_storage_location::<T>))
        // .route("/storage_locations/{name}", patch(update_storage_location::<T>))
        .route(
            "/storage_locations/{name}",
            delete(delete_storage_location::<T>),
        )
        .with_state(handler)
}

async fn create_credential<T: CredentialsHandler>(
    State(handler): State<T>,
    Extension(recipient): Extension<Recipient>,
    request: CreateCredentialRequest,
) -> Result<Json<Credential>> {
    let ctx = RequestContext { recipient };
    Ok(Json(handler.create_credential(request, ctx).await?))
}

async fn get_credential<T: CredentialsHandler>(
    State(handler): State<T>,
    Extension(recipient): Extension<Recipient>,
    request: GetCredentialRequest,
) -> Result<Json<Credential>> {
    let ctx = RequestContext { recipient };
    Ok(Json(handler.get_credential(request, ctx).await?))
}

async fn delete_credential<T: CredentialsHandler>(
    State(handler): State<T>,
    Extension(recipient): Extension<Recipient>,
    request: DeleteCredentialRequest,
) -> Result<Json<()>> {
    let ctx = RequestContext { recipient };
    Ok(Json(handler.delete_credential(request, ctx).await?))
}

async fn create_storage_location<T: CredentialsHandler>(
    State(handler): State<T>,
    Extension(recipient): Extension<Recipient>,
    request: CreateStorageLocationRequest,
) -> Result<Json<StorageLocation>> {
    let ctx = RequestContext { recipient };
    Ok(Json(handler.create_storage_location(request, ctx).await?))
}

async fn list_storage_locations<T: CredentialsHandler>(
    State(handler): State<T>,
    Extension(recipient): Extension<Recipient>,
    request: ListStorageLocationsRequest,
) -> Result<Json<ListStorageLocationsResponse>> {
    let ctx = RequestContext { recipient };
    Ok(Json(handler.list_storage_locations(request, ctx).await?))
}

async fn get_storage_location<T: CredentialsHandler>(
    State(handler): State<T>,
    Extension(recipient): Extension<Recipient>,
    request: GetStorageLocationRequest,
) -> Result<Json<StorageLocation>> {
    let ctx = RequestContext { recipient };
    Ok(Json(handler.get_storage_location(request, ctx).await?))
}

async fn delete_storage_location<T: CredentialsHandler>(
    State(handler): State<T>,
    Extension(recipient): Extension<Recipient>,
    request: DeleteStorageLocationRequest,
) -> Result<Json<()>> {
    let ctx = RequestContext { recipient };
    Ok(Json(handler.delete_storage_location(request, ctx).await?))
}
