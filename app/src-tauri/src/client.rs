use delta_sharing_common::models::catalogs::v1::{CatalogInfo, CreateCatalogRequest};
use delta_sharing_common::models::schemas::v1::{CreateSchemaRequest, SchemaInfo};
use delta_sharing_common::rest::client::UnityCatalogClient;
use futures::TryStreamExt;
use tauri::State;

use crate::error::Result;

#[tauri::command]
pub async fn list_catalogs(
    state: State<'_, UnityCatalogClient>,
    max_results: Option<i32>,
) -> Result<Vec<CatalogInfo>> {
    Ok(state.catalogs().list(max_results).try_collect().await?)
}

#[tauri::command]
pub async fn get_catalog(
    state: State<'_, UnityCatalogClient>,
    name: String,
) -> Result<CatalogInfo> {
    Ok(state.catalogs().get(name).await?)
}

#[tauri::command]
pub async fn create_catalog(
    state: State<'_, UnityCatalogClient>,
    request: CreateCatalogRequest,
) -> Result<CatalogInfo> {
    Ok(state.catalogs().create_catalog(&request).await?)
}

#[tauri::command]
pub async fn delete_catalog(
    state: State<'_, UnityCatalogClient>,
    name: String,
    force: Option<bool>,
) -> Result<()> {
    Ok(state.catalogs().delete(name, force).await?)
}

#[tauri::command]
pub async fn list_schemas(
    state: State<'_, UnityCatalogClient>,
    catalog: String,
    max_results: Option<i32>,
) -> Result<Vec<SchemaInfo>> {
    Ok(state
        .schemas()
        .list(catalog, max_results)
        .try_collect()
        .await?)
}

#[tauri::command]
pub async fn get_schema(
    state: State<'_, UnityCatalogClient>,
    catalog: String,
    schema: String,
) -> Result<SchemaInfo> {
    Ok(state.schemas().get(catalog, schema).await?)
}

#[tauri::command]
pub async fn create_schema(
    state: State<'_, UnityCatalogClient>,
    request: CreateSchemaRequest,
) -> Result<SchemaInfo> {
    Ok(state.schemas().create_schema(&request).await?)
}

#[tauri::command]
pub async fn delete_schema(
    state: State<'_, UnityCatalogClient>,
    catalog: String,
    name: String,
    force: Option<bool>,
) -> Result<()> {
    Ok(state.schemas().delete(catalog, name, force).await?)
}
