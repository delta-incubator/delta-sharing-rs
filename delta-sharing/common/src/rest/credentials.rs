use axum::routing::{delete, get, post};
use axum::{RequestExt, RequestPartsExt, Router};
use delta_sharing_derive::rest_handlers;

use crate::api::{CredentialsHandler, RequestContext};
use crate::models::credentials::v1::*;
use crate::{Error, Recipient, Result};

pub fn get_router<T: CredentialsHandler + Clone>(handler: T) -> Router {
    Router::new()
        .route("/credentials", post(create_credential::<T>))
        .route("/credentials/{name}", get(get_credential::<T>))
        //.route("/credentials/{name}", patch(update_credential::<T>))
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

rest_handlers!(
    CredentialsHandler,
    [
        CreateCredentialRequest, Credential;
        GetCredentialRequest, Credential with [
            name: path as String,
        ];
        DeleteCredentialRequest with [
            name: path as String,
        ];
        CreateStorageLocationRequest, StorageLocation;
        ListStorageLocationsRequest, ListStorageLocationsResponse;
        GetStorageLocationRequest, StorageLocation with [
            name: path as String,
        ];
        DeleteStorageLocationRequest with [
            name: path as String,
        ];
    ]
);
