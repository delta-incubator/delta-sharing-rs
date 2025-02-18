use axum::extract::{Extension, State};
use axum::{
    routing::{delete, post},
    Json, Router,
};

use crate::models::catalog::v1::*;
use crate::{Recipient, RepositoryManager, Result};

/// Create a new [Router] for the Delta Sharing REST API.
pub fn get_router<T: RepositoryManager + Clone>(handler: T) -> Router {
    Router::new()
        .route("/shares", post(create_share::<T>))
        .route("/shares/{share}", delete(delete_share::<T>))
        .route("/shares/{share}/schemas", post(create_schema::<T>))
        .route("/shares/{share}/schemas/{name}", delete(delete_schema::<T>))
        .with_state(handler)
}

async fn create_share<T: RepositoryManager>(
    State(handler): State<T>,
    Extension(recipient): Extension<Recipient>,
    request: CreateShareRequest,
) -> Result<Json<ShareInfo>> {
    let share = handler.create_share(request, &recipient).await?;
    Ok(Json(share))
}

async fn delete_share<T: RepositoryManager>(
    State(handler): State<T>,
    Extension(recipient): Extension<Recipient>,
    request: DeleteShareRequest,
) -> Result<Json<()>> {
    handler.delete_share(request, &recipient).await?;
    Ok(Json(()))
}

async fn create_schema<T: RepositoryManager>(
    State(handler): State<T>,
    Extension(recipient): Extension<Recipient>,
    request: CreateSchemaRequest,
) -> Result<Json<SchemaInfo>> {
    let schema = handler.create_schema(request, &recipient).await?;
    Ok(Json(schema))
}

async fn delete_schema<T: RepositoryManager>(
    State(handler): State<T>,
    Extension(recipient): Extension<Recipient>,
    request: DeleteSchemaRequest,
) -> Result<Json<()>> {
    handler.delete_schema(request, &recipient).await?;
    Ok(Json(()))
}
