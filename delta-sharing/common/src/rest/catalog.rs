use axum::extract::{Extension, State};
use axum::{
    routing::{delete, get, patch, post},
    Json, Router,
};

use crate::api::{CatalogHandler, RequestContext};
use crate::models::catalog::v1::*;
use crate::{Recipient, Result};

/// Create a new [Router] for the Delta Sharing REST API.
pub fn get_router<T: CatalogHandler + Clone>(handler: T) -> Router {
    Router::new()
        .route("/catalogs", post(create_catalog::<T>))
        .route("/catalogs", get(list_catalogs::<T>))
        .route("/catalogs/{full_name}", get(get_catalog::<T>))
        .route("/catalogs/{full_name}", delete(delete_catalog::<T>))
        .route("/catalogs/{full_name}", patch(update_catalog::<T>))
        .route("/schemas", post(create_schema::<T>))
        .route("/schemas", get(list_schemas::<T>))
        .route("/schemas/{full_name}", delete(delete_schema::<T>))
        .route("/schemas/{full_name}", patch(update_schema::<T>))
        .with_state(handler)
}

async fn create_catalog<T: CatalogHandler>(
    State(handler): State<T>,
    Extension(recipient): Extension<Recipient>,
    request: CreateCatalogRequest,
) -> Result<Json<CatalogInfo>> {
    let ctx = RequestContext { recipient };
    Ok(Json(handler.create_catalog(request, ctx).await?))
}

async fn list_catalogs<T: CatalogHandler>(
    State(handler): State<T>,
    Extension(recipient): Extension<Recipient>,
    request: ListCatalogsRequest,
) -> Result<Json<ListCatalogsResponse>> {
    let ctx = RequestContext { recipient };
    Ok(Json(handler.list_catalogs(request, ctx).await?))
}

async fn get_catalog<T: CatalogHandler>(
    State(handler): State<T>,
    Extension(recipient): Extension<Recipient>,
    request: GetCatalogRequest,
) -> Result<Json<CatalogInfo>> {
    let ctx = RequestContext { recipient };
    Ok(Json(handler.get_catalog(request, ctx).await?))
}

async fn delete_catalog<T: CatalogHandler>(
    State(handler): State<T>,
    Extension(recipient): Extension<Recipient>,
    request: DeleteCatalogRequest,
) -> Result<()> {
    let ctx = RequestContext { recipient };
    handler.delete_catalog(request, ctx).await?;
    Ok(())
}

async fn update_catalog<T: CatalogHandler>(
    State(handler): State<T>,
    Extension(recipient): Extension<Recipient>,
    request: UpdateCatalogRequest,
) -> Result<Json<CatalogInfo>> {
    let ctx = RequestContext { recipient };
    Ok(Json(handler.update_catalog(request, ctx).await?))
}

async fn create_schema<T: CatalogHandler>(
    State(handler): State<T>,
    Extension(recipient): Extension<Recipient>,
    request: CreateSchemaRequest,
) -> Result<Json<SchemaInfo>> {
    let ctx = RequestContext { recipient };
    Ok(Json(handler.create_schema(request, ctx).await?))
}

async fn list_schemas<T: CatalogHandler>(
    State(handler): State<T>,
    Extension(recipient): Extension<Recipient>,
    request: ListSchemasRequest,
) -> Result<Json<ListSchemasResponse>> {
    let ctx = RequestContext { recipient };
    Ok(Json(handler.list_schemas(request, ctx).await?))
}

async fn delete_schema<T: CatalogHandler>(
    State(handler): State<T>,
    Extension(recipient): Extension<Recipient>,
    request: DeleteSchemaRequest,
) -> Result<()> {
    let ctx = RequestContext { recipient };
    handler.delete_schema(request, ctx).await?;
    Ok(())
}

async fn update_schema<T: CatalogHandler>(
    State(handler): State<T>,
    Extension(recipient): Extension<Recipient>,
    request: UpdateSchemaRequest,
) -> Result<Json<SchemaInfo>> {
    let ctx = RequestContext { recipient };
    Ok(Json(handler.update_schema(request, ctx).await?))
}
