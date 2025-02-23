use axum::routing::{delete, get, patch, post, Router};

use crate::api::catalog::*;

/// Create a new [Router] for the Delta Sharing REST API.
pub fn get_router<T: CatalogHandler + Clone>(handler: T) -> Router {
    Router::new()
        .route("/catalogs", post(create_catalog::<T>))
        .route("/catalogs", get(list_catalogs::<T>))
        .route("/catalogs/{name}", get(get_catalog::<T>))
        .route("/catalogs/{name}", patch(update_catalog::<T>))
        .route("/catalogs/{name}", delete(delete_catalog::<T>))
        .route("/schemas", post(create_schema::<T>))
        .route("/schemas", get(list_schemas::<T>))
        .route("/schemas/{full_name}", get(get_schema::<T>))
        .route("/schemas/{full_name}", delete(delete_schema::<T>))
        .route("/schemas/{full_name}", patch(update_schema::<T>))
        .with_state(handler)
}
