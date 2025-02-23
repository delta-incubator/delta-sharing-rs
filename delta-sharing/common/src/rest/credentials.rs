use axum::routing::{delete, get, post, Router};

use crate::api::credentials::*;
use crate::api::CredentialsHandler;

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
